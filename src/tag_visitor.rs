use verus_syn::*;
use syn::visit::Visit;
use proc_macro2::Span;

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub line: usize,
    pub byte_offset: usize,
    pub pattern: String,
}

#[derive(Debug)]
pub struct TagVisitor<'a> {
    source: &'a str,
    tags: Vec<Tag>,
}

impl<'a> TagVisitor<'a> {
    pub fn new(source: &'a str) -> Self {
        TagVisitor {
            source,
            tags: Vec::new(),
        }
    }

    pub fn visit_file(&mut self, file: &'a File) {
        syn::visit::visit_file(self, file);
    }

    pub fn tags(self) -> Vec<Tag> {
        self.tags
    }

    /// Process verus! and verus_! macro invocations to extract spec/proof/exec functions
    pub fn process_verus_macros(&mut self, file: &File) {
        for item in &file.items {
            if let Item::Macro(item_macro) = item {
                // Check if this is a verus!, verus_!, or verus_impl! macro
                let ident = item_macro.mac.path.segments.last().map(|seg| &seg.ident);
                let is_verus_macro = ident.map(|id| {
                    id == "verus" || id == "verus_" || id == "verus_impl"
                }).unwrap_or(false);
                
                if is_verus_macro {
                    // Try to parse the macro contents as a File
                    if let Ok(inner_file) = syn::parse2::<File>(item_macro.mac.tokens.clone()) {
                        // Manually process items from the verus! macro
                        self.process_verus_items(&inner_file.items);
                    }
                }
            }
        }
    }

    fn process_verus_items(&mut self, items: &[Item]) {
        for item in items {
            match item {
                Item::Fn(item_fn) => {
                    let name = item_fn.sig.ident.to_string();
                    self.add_tag(name, item_fn.sig.ident.span());
                }
                Item::Struct(item_struct) => {
                    let name = item_struct.ident.to_string();
                    self.add_tag(name, item_struct.ident.span());
                }
                Item::Enum(item_enum) => {
                    let name = item_enum.ident.to_string();
                    self.add_tag(name.clone(), item_enum.ident.span());
                    for variant in &item_enum.variants {
                        let variant_name = format!("{}::{}", name, variant.ident);
                        self.add_tag(variant_name, variant.ident.span());
                    }
                }
                Item::Trait(item_trait) => {
                    let name = item_trait.ident.to_string();
                    self.add_tag(name, item_trait.ident.span());
                }
                Item::Impl(item_impl) => {
                    if let Type::Path(type_path) = &*item_impl.self_ty {
                        if let Some(segment) = type_path.path.segments.last() {
                            let impl_name = if let Some((_, trait_path, _)) = &item_impl.trait_ {
                                if let Some(trait_segment) = trait_path.segments.last() {
                                    format!("impl {} for {}", trait_segment.ident, segment.ident)
                                } else {
                                    format!("impl {}", segment.ident)
                                }
                            } else {
                                format!("impl {}", segment.ident)
                            };
                            self.add_tag(impl_name, segment.ident.span());
                        }
                    }
                    
                    // Process impl items
                    for impl_item in &item_impl.items {
                        if let ImplItem::Fn(impl_item_fn) = impl_item {
                            let name = impl_item_fn.sig.ident.to_string();
                            self.add_tag(name, impl_item_fn.sig.ident.span());
                        }
                    }
                }
                Item::Const(item_const) => {
                    let name = item_const.ident.to_string();
                    self.add_tag(name, item_const.ident.span());
                }
                Item::Static(item_static) => {
                    let name = item_static.ident.to_string();
                    self.add_tag(name, item_static.ident.span());
                }
                Item::Type(item_type) => {
                    let name = item_type.ident.to_string();
                    self.add_tag(name, item_type.ident.span());
                }
                Item::Mod(item_mod) => {
                    let name = item_mod.ident.to_string();
                    self.add_tag(name, item_mod.ident.span());
                }
                Item::BroadcastGroup(item_bg) => {
                    let name = item_bg.ident.to_string();
                    self.add_tag(name, item_bg.ident.span());
                }
                _ => {}
            }
        }
    }

    fn add_tag(&mut self, name: String, span: Span) {
        let start = span.start();
        let line = start.line;
        let byte_offset = self.get_line_start_offset(line);
        let pattern = self.extract_pattern(byte_offset, &name);
        
        self.tags.push(Tag {
            name,
            line,
            byte_offset,
            pattern,
        });
    }

    fn get_line_start_offset(&self, line_num: usize) -> usize {
        if line_num == 1 {
            return 0;
        }

        let mut current_line = 1;
        for (idx, ch) in self.source.char_indices() {
            if ch == '\n' {
                current_line += 1;
                if current_line == line_num {
                    return idx + 1;
                }
            }
        }

        0
    }

    fn extract_pattern(&self, byte_offset: usize, name: &str) -> String {
        // Find the line containing the definition
        let line_start = self.source[..byte_offset]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        
        let line_end = self.source[byte_offset..]
            .find('\n')
            .map(|pos| byte_offset + pos)
            .unwrap_or(self.source.len());

        let line = &self.source[line_start..line_end];
        
        // For multi-line signatures, just use the identifier name as the pattern
        // to avoid matching issues. Emacs will use line+offset to find the exact location.
        let trimmed = line.trim();
        if trimmed.ends_with('(') || (trimmed.contains('(') && !trimmed.contains(')')) {
            // Multi-line function signature, use just the name
            name.to_string()
        } else {
            // Single line, use the whole line WITH INDENTATION (like ctags does)
            // Remove only trailing whitespace, keep leading indentation
            line.trim_end().to_string()
        }
    }

    fn extract_fn_mode(fn_mode: &FnMode) -> Option<&'static str> {
        match fn_mode {
            FnMode::Spec(_) => Some("spec"),
            FnMode::SpecChecked(_) => Some("spec(checked)"),
            FnMode::Proof(_) => Some("proof"),
            FnMode::ProofAxiom(_) => Some("axiom"),
            FnMode::Exec(_) => Some("exec"),
            FnMode::Default => None,
        }
    }
}

impl<'a> Visit<'a> for TagVisitor<'a> {
    fn visit_item_fn(&mut self, node: &'a ItemFn) {
        let name = node.sig.ident.to_string();
        self.add_tag(name, node.sig.ident.span());
        
        // Continue visiting nested items
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_struct(&mut self, node: &'a ItemStruct) {
        let name = node.ident.to_string();
        self.add_tag(name, node.ident.span());
        
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_enum(&mut self, node: &'a ItemEnum) {
        let name = node.ident.to_string();
        self.add_tag(name.clone(), node.ident.span());
        
        // Add enum variants
        for variant in &node.variants {
            let variant_name = format!("{}::{}", name, variant.ident);
            self.add_tag(variant_name, variant.ident.span());
        }
        
        syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_trait(&mut self, node: &'a ItemTrait) {
        let name = node.ident.to_string();
        self.add_tag(name, node.ident.span());
        
        syn::visit::visit_item_trait(self, node);
    }

    fn visit_item_impl(&mut self, node: &'a ItemImpl) {
        // For impl blocks, we tag the type being implemented
        if let Type::Path(type_path) = &*node.self_ty {
            if let Some(segment) = type_path.path.segments.last() {
                let impl_name = if let Some((_, trait_path, _)) = &node.trait_ {
                    // Trait implementation
                    if let Some(trait_segment) = trait_path.segments.last() {
                        format!("impl {} for {}", trait_segment.ident, segment.ident)
                    } else {
                        format!("impl {}", segment.ident)
                    }
                } else {
                    // Inherent impl
                    format!("impl {}", segment.ident)
                };
                
                self.add_tag(impl_name, segment.ident.span());
            }
        }
        
        syn::visit::visit_item_impl(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'a ImplItemFn) {
        let name = node.sig.ident.to_string();
        self.add_tag(name, node.sig.ident.span());
        
        syn::visit::visit_impl_item_fn(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &'a TraitItemFn) {
        let name = node.sig.ident.to_string();
        self.add_tag(name, node.sig.ident.span());
        
        syn::visit::visit_trait_item_fn(self, node);
    }

    fn visit_item_type(&mut self, node: &'a ItemType) {
        let name = node.ident.to_string();
        self.add_tag(name, node.ident.span());
        
        syn::visit::visit_item_type(self, node);
    }

    fn visit_item_const(&mut self, node: &'a ItemConst) {
        let name = node.ident.to_string();
        self.add_tag(name, node.ident.span());
        
        syn::visit::visit_item_const(self, node);
    }

    fn visit_item_static(&mut self, node: &'a ItemStatic) {
        let name = node.ident.to_string();
        self.add_tag(name, node.ident.span());
        
        syn::visit::visit_item_static(self, node);
    }

    fn visit_item_mod(&mut self, node: &'a ItemMod) {
        let name = node.ident.to_string();
        self.add_tag(name, node.ident.span());
        
        syn::visit::visit_item_mod(self, node);
    }

    fn visit_item_macro(&mut self, node: &'a ItemMacro) {
        if let Some(ident) = &node.ident {
            let name = ident.to_string();
            self.add_tag(name, ident.span());
        }
        
        syn::visit::visit_item_macro(self, node);
    }

    fn visit_item_broadcast_group(&mut self, node: &'a ItemBroadcastGroup) {
        let name = node.ident.to_string();
        self.add_tag(name, node.ident.span());
        
        syn::visit::visit_item_broadcast_group(self, node);
    }

    fn visit_assume_specification(&mut self, node: &'a AssumeSpecification) {
        // Extract the function name from the path
        if let Some(segment) = node.path.segments.last() {
            let name = format!("assume_specification {}", segment.ident);
            self.add_tag(name, segment.ident.span());
        }
        
        syn::visit::visit_assume_specification(self, node);
    }
}

