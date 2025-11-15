use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod tag_visitor;
use tag_visitor::{Tag, TagVisitor};

#[derive(Parser, Debug)]
#[command(name = "verus-etags")]
#[command(version)]
#[command(about = "Generate etags for Verus/Rust source files", long_about = None)]
#[command(disable_version_flag = true)]
struct Args {
    /// Print version
    #[arg(short = 'v', long = "version")]
    version: bool,

    /// Input files or directories to process
    #[arg(required_unless_present = "version")]
    paths: Vec<PathBuf>,

    /// Output file (default: TAGS)
    #[arg(short = 'o', long, visible_alias = "file", short_alias = 'f', default_value = "TAGS")]
    output: PathBuf,

    /// Append to existing tags file instead of overwriting
    #[arg(short, long)]
    append: bool,

    /// Recurse into directories (default: true, use --no-recurse to disable)
    #[arg(short = 'R', long, default_value_t = true, action = clap::ArgAction::SetTrue)]
    recurse: bool,
    
    /// Do not recurse into subdirectories
    #[arg(long, conflicts_with = "recurse")]
    no_recurse: bool,

    /// Verbose output
    #[arg(short = 'V', long, visible_alias = "verbose")]
    verbose_mode: bool,

    /// Sort tags (0=unsorted, 1=sorted, 2=foldcase)
    #[arg(short, long, value_name = "0|1|2", default_value = "1")]
    sort: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle version flag manually
    if args.version {
        println!("verus-etags {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let mut all_tags: Vec<(PathBuf, Vec<Tag>)> = Vec::new();

    // If append mode, load existing tags
    if args.append && args.output.exists() {
        if args.verbose_mode {
            eprintln!("Appending to existing tags file: {}", args.output.display());
        }
        all_tags = read_existing_tags(&args.output)?;
    }

    // Determine if we should recurse
    let should_recurse = args.recurse && !args.no_recurse;

    // Collect all Rust files
    for path in &args.paths {
        if path.is_file() {
            if is_rust_file(path) {
                if args.verbose_mode {
                    eprintln!("Processing file: {}", path.display());
                }
                match process_file(path) {
                    Ok(tags) => all_tags.push((path.clone(), tags)),
                    Err(e) => {
                        if args.verbose_mode {
                            eprintln!("Warning: Skipping file {}: {}", path.display(), e);
                        }
                    }
                }
            }
        } else if path.is_dir() {
            if should_recurse {
                // Recursive directory traversal
                for entry in WalkDir::new(path)
                    .follow_links(true)
                    .into_iter()
                    .filter_entry(|e| {
                        // Skip hidden directories (but not the root ".")
                        // Skip Emacs temp files
                        if let Some(name) = e.file_name().to_str() {
                            // Don't filter the current directory "."
                            if name == "." {
                                return true;
                            }
                            // Filter hidden dirs/files and Emacs temp files
                            if name.starts_with(".#") || name.ends_with('~') || 
                               (name.starts_with('#') && name.ends_with('#')) || 
                               name.starts_with('.') {
                                return false;
                            }
                        }
                        true
                    })
                {
                    let entry = match entry {
                        Ok(e) => e,
                        Err(_) => continue, // Skip entries that can't be read (broken symlinks, etc.)
                    };
                    let file_path = entry.path();
                    if file_path.is_file() && is_rust_file(file_path) {
                        if args.verbose_mode {
                            eprintln!("Processing file: {}", file_path.display());
                        }
                        match process_file(file_path) {
                            Ok(tags) => all_tags.push((file_path.to_path_buf(), tags)),
                            Err(e) => {
                                if args.verbose_mode {
                                    eprintln!("Warning: Skipping file {}: {}", file_path.display(), e);
                                }
                            }
                        }
                    }
                }
            } else {
                // Non-recursive: only process files in the immediate directory
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let file_path = entry.path();
                        if file_path.is_file() && is_rust_file(&file_path) {
                            if args.verbose_mode {
                                eprintln!("Processing file: {}", file_path.display());
                            }
                            match process_file(&file_path) {
                                Ok(tags) => all_tags.push((file_path, tags)),
                                Err(e) => {
                                    if args.verbose_mode {
                                        eprintln!("Warning: Skipping file {}: {}", file_path.display(), e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort tags if requested
    if args.sort > 0 {
        sort_tags(&mut all_tags, args.sort == 2);
    }

    // Write etags format
    write_etags(&args.output, &all_tags, args.append)?;

    if args.verbose_mode {
        eprintln!("Generated {} with {} files", args.output.display(), all_tags.len());
    }

    Ok(())
}

fn is_rust_file(path: &Path) -> bool {
    // Skip Emacs temporary files
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        // Skip .#filename (lock files), *~ (backup files), #*# (auto-save files)
        if name.starts_with(".#") || name.ends_with('~') || (name.starts_with('#') && name.ends_with('#')) {
            return false;
        }
    }
    
    path.extension().map_or(false, |ext| ext == "rs")
}

fn process_file(path: &Path) -> Result<Vec<Tag>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    // Try verus_syn first (handles Verus-specific syntax)
    if let Ok(syntax_tree) = verus_syn::parse_file(&content) {
        let mut visitor = TagVisitor::new(&content);
        visitor.visit_file(&syntax_tree);
        
        // Also try to extract tags from verus! macro invocations
        visitor.process_verus_macros(&syntax_tree);

        return Ok(visitor.tags());
    }

    // Fall back to regular syn parser (for pure Rust files like compiler internals)
    let syntax_tree = syn::parse_file(&content)
        .with_context(|| format!("Failed to parse file with both verus_syn and syn: {}", path.display()))?;

    let mut visitor = TagVisitor::new(&content);
    visitor.visit_file_regular_syn(&syntax_tree);

    Ok(visitor.tags())
}

fn sort_tags(all_tags: &mut Vec<(PathBuf, Vec<Tag>)>, foldcase: bool) {
    for (_path, tags) in all_tags.iter_mut() {
        tags.sort_by(|a, b| {
            // Primary sort: by line number (for etags format compatibility)
            // Secondary sort: by name (with optional case-folding)
            match a.line.cmp(&b.line) {
                std::cmp::Ordering::Equal => {
                    if foldcase {
                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                    } else {
                        a.name.cmp(&b.name)
                    }
                }
                other => other,
            }
        });
    }
}

fn write_etags(output_path: &Path, all_tags: &[(PathBuf, Vec<Tag>)], _append: bool) -> Result<()> {
    let mut file = fs::File::create(output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

    for (file_path, tags) in all_tags {
        if tags.is_empty() {
            continue;
        }

        // Calculate the size of this file's tag section
        let mut section_content = Vec::new();
        for tag in tags {
            // Format: <pattern>\x7f<tagname>\x01<line>,<byte_offset>\n
            writeln!(
                section_content,
                "{}\x7f{}\x01{},{}",
                tag.pattern, tag.name, tag.line, tag.byte_offset
            )?;
        }

        let section_size = section_content.len();

        // Write the file header: \x0c\n<filename>,<section_size>\n
        write!(file, "\x0c\n{},{}\n", file_path.display(), section_size)?;

        // Write the tag entries
        file.write_all(&section_content)?;
    }

    Ok(())
}

fn read_existing_tags(_tags_file: &Path) -> Result<Vec<(PathBuf, Vec<Tag>)>> {
    // TODO: Implement parsing existing tags file for append mode
    // For now, return empty vec (append mode will just overwrite)
    Ok(Vec::new())
}
