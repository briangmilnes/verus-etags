# verus-etags

An AST-based etags generator for Verus (verified Rust) source code. This tool parses Verus/Rust code using the Verus-extended `syn` parser and generates Emacs-compatible TAGS files for code navigation with perfect `M-.` (xref) integration.

## Features

- **Pure AST-based parsing**: No string hacking - uses proper syntax tree parsing via `syn::visit::Visit` trait
- **Verus-aware**: Fully understands Verus-specific syntax including:
  - Function modes: `spec fn`, `proof fn`, `exec fn`, `spec(checked) fn`
  - Const modes: `spec const`, `exec const`, `proof const`
  - Function modifiers: `pub open spec fn`, `pub open exec fn`
  - Data modes: `ghost`, `tracked`
  - Verus specifications: `requires`, `ensures`, `invariant`, `recommends`, etc.
  - Broadcast groups and axioms
  - Assume specifications
  - External trait specifications
- **Macro parsing**: Automatically extracts items from Verus macro blocks:
  - `verus! { }` - Main Verus macro
  - `verus_! { }` - Alternative naming (used in std_specs)
  - `verus_impl! { }` - Implementation macro (used in atomic operations)
- **Comprehensive tagging**: Generates tags for all Rust/Verus constructs:
  - Functions (all modes: spec, proof, exec)
  - Structs
  - Enums (including variants)
  - Traits (including trait methods)
  - Impl blocks (including impl methods)
  - Type aliases
  - Constants (all modes)
  - Static variables
  - Modules
  - Macros
  - Broadcast groups
- **Emacs xref compatible**: 
  - Accurate byte offsets pointing to line starts
  - Preserves source indentation in patterns
  - Works seamlessly with `M-.` (xref-find-definitions)
- **Smart file filtering**: Automatically ignores Emacs temp files (`.#*`, `*~`, `#*#`)
- **ctags-compatible CLI**: Drop-in replacement for common ctags/etags workflows

## Installation

```bash
cargo build --release
```

The binary will be available at `./target/release/verus-etags`.

## Usage

```bash
verus-etags [OPTIONS] <PATHS>...
```

### Arguments

- `<PATHS>...` - Input files or directories to process (required unless using `--version`)

### Options

- `-v, --version` - Print version information
- `-o, --output <OUTPUT>` - Output file (default: TAGS) [aliases: `-f`, `--file`]
- `-a, --append` - Append to existing tags file instead of overwriting
- `-R, --recurse` - Recurse into directories (default: true)
- `--no-recurse` - Do not recurse into subdirectories
- `-V, --verbose` - Verbose output (shows each file being processed)
- `-s, --sort <0|1|2>` - Sort tags (0=unsorted, 1=sorted, 2=foldcase) [default: 1]
- `-h, --help` - Print help

### Examples

Generate tags for a single file:
```bash
verus-etags src/main.rs
```

Generate tags for an entire project (recursive):
```bash
verus-etags src/
```

Generate tags for the Verus standard library (vstd):
```bash
cd ~/verus/source
verus-etags -o TAGS vstd/
# Generates 2200+ tags from 85 files
```

Multiple directories:
```bash
verus-etags src/ tests/ benches/
```

Custom output file:
```bash
verus-etags -o TAGS.verus src/
```

Append new tags to existing file:
```bash
verus-etags -a -o TAGS src/new_module.rs
```

Verbose mode (see which files are processed):
```bash
verus-etags -V src/
```

Non-recursive (only immediate directory):
```bash
verus-etags --no-recurse src/
```

## Compatibility

The command-line interface matches common ctags/etags conventions:

| Option | Alias | Description |
|--------|-------|-------------|
| `-v` | `--version` | Show version (matches ctags) |
| `-V` | `--verbose` | Verbose output (matches ctags) |
| `-f <file>` | `--file`, `-o`, `--output` | Output file |
| `-a` | `--append` | Append mode |
| `-R` | `--recurse` | Recursive traversal |
| `--no-recurse` | | Non-recursive |
| `-s <0\|1\|2>` | `--sort` | Sort tags |

## Requirements

- **Rust 1.88.0** (specified in `rust-toolchain.toml`)
- **Verus repository**: Cloned in `verus-lang/` subdirectory for the Verus-extended `syn` parser

The tool automatically uses the Verus-extended syntax parser from the included `verus-lang/` repository, which understands all Verus language extensions.

## How It Works

1. **File Discovery**: Walks through specified files/directories, filtering out:
   - Emacs lock files (`.#filename`)
   - Backup files (`*~`)
   - Auto-save files (`#*#`)
   - Hidden files and directories (`.git`, etc.)

2. **AST Parsing**: For each Rust file, parses using `verus_syn::parse_file()`

3. **AST Traversal**: A `syn::visit::Visit` implementation walks the syntax tree extracting:
   - Symbol names (identifiers)
   - Line numbers
   - Byte offsets (pointing to line start for Emacs xref)
   - Source patterns (preserving indentation)

4. **Macro Expansion**: For `verus!`, `verus_!`, and `verus_impl!` macros:
   - Parses the macro's token stream as a nested syntax tree
   - Recursively extracts all items within the macro
   - Preserves all Verus mode annotations

5. **Tag Sorting**: Tags are sorted by line number within each file (required for efficient Emacs xref lookup)

6. **Tag Generation**: Writes tags in Emacs etags format with:
   - Clean tag names (e.g., `my_function`, not `my_function (spec)`)
   - Full source patterns for matching
   - Accurate byte offsets for navigation
   - Tags ordered by line number for optimal Emacs performance

## Etags Format

The generated TAGS file uses the standard Emacs etags format:
```
^L
<filename>,<section_size>
<pattern>^?<tagname>^A<line>,<byte_offset>
...
```

Where:
- `^L` is the form feed character (0x0C) - section separator
- `^?` is DEL (0x7F) - separates pattern from tagname
- `^A` is SOH (0x01) - separates tagname from location

Example entry:
```
spec fn my_spec(x: int) -> int {^?my_spec^A94,3324
```

This means: at line 94, byte offset 3324, find `my_spec` by matching the pattern `spec fn my_spec(x: int) -> int {`

## Emacs Integration

### Using with Emacs

After generating TAGS:

```elisp
;; In your .emacs or init.el
(setq tags-table-list '("~/my-project/TAGS"))

;; Or interactively:
M-x visit-tags-table RET ~/my-project/TAGS RET
```

Then use standard Emacs navigation:
- `M-.` - Jump to definition (xref-find-definitions)
- `M-,` - Pop back
- `M-x tags-search` - Search across tagged files

### Regenerating Tags

Add to your Makefile or build script:
```makefile
tags:
	verus-etags -o TAGS src/ tests/
```

Or use a file watcher to regenerate on save.

## Testing

The project includes comprehensive tests (33 tests total):

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Test specific area
cargo test byte_offset
cargo test verus_syntax
cargo test cli_switches
cargo test tag_ordering
```

Test coverage includes:
- All CLI switches and aliases
- Verus syntax variants (`spec fn`, `proof fn`, `exec const`, `pub open spec`, etc.)
- `verus!`, `verus_!`, `verus_impl!` macro parsing
- Byte offset accuracy for Emacs xref
- Pattern matching for multi-line declarations
- Emacs temp file filtering
- Append mode and sorting
- Tag line-number ordering (critical for Emacs xref performance)

## Verified on Real Projects

Successfully tested on:
- **Verus examples** (~/verus/examples/): 144 files, 1700+ lines of TAGS
- **Verus stdlib (vstd)**: 85 files, 2207 tags, all Verus language features
- **APAS-AI project**: Large Verus codebase with complex module structure

## Performance

- **Fast**: Processes vstd (85 files, ~25K LOC) in < 1 second
- **Memory efficient**: Streams processing, doesn't load entire codebase at once
- **Incremental**: Use `-a` (append) for fast incremental updates

## Development

Build:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Test on Verus examples:
```bash
cargo run -- ~/verus/examples/
```

Check for string hacking (should be none!):
```bash
~/projects/rusticate/target/release/rusticate-review-string-hacking -c
# Output: 0 violations in verus-etags source
```

## Comparison with ctags

| Feature | ctags | verus-etags |
|---------|-------|-------------|
| Verus syntax (`spec fn`, `proof fn`) | ❌ No | ✅ Yes |
| `verus!` macro parsing | ❌ No | ✅ Yes |
| AST-based | ❌ Regex | ✅ Full AST |
| Verus stdlib (vstd) | ⚠️ 2039 tags | ✅ 2207 tags |
| Emacs xref compatible | ✅ Yes | ✅ Yes |
| CLI compatible | ✅ Standard | ✅ Compatible |

## Known Limitations

- **Macro expansion**: Only parses `verus!`, `verus_!`, and `verus_impl!` macros. Other macro invocations are tagged as macro calls but their contents aren't expanded.
- **Cross-file references**: Like all etags tools, only generates tags for symbols, not cross-references.
- **Conditional compilation**: Doesn't evaluate `#[cfg(...)]` attributes; tags all code paths.

## Author

**Brian Milnes** - <briangmilnes@gmail.com>

## License

MIT

## Contributing

Contributions welcome! Please ensure:
1. All tests pass: `cargo test`
2. No string hacking: Code uses AST traversal only
3. Emacs xref compatibility maintained
4. CLI compatibility with ctags conventions

## Acknowledgments

- Built using the [Verus](https://github.com/verus-lang/verus) extended `syn` parser
- Inspired by the need for better Verus code navigation in Emacs
- Tested on real-world Verus codebases including APAS-AI
