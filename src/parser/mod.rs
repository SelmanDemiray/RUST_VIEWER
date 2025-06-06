pub mod types;
pub mod element;
pub mod relationship;
pub mod analyzer;

pub use types::*;
pub use relationship::ParseRelationship;
pub use analyzer::*;

use syn::{visit::Visit, ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemImpl, ItemMod};
use quote::ToTokens; // Add this import

// Main parsing function
pub fn parse_file(file_path: &str, content: &str) -> Result<(Vec<CodeElement>, Vec<ParseRelationship>), String> {
    // Parse the file using syn
    match syn::parse_file(content) {
        Ok(syntax_tree) => {
            let mut visitor = CodeVisitor::new(file_path.to_string());
            visitor.visit_file(&syntax_tree);
            Ok((visitor.elements, visitor.relationships))
        }
        Err(e) => {
            Err(format!("Failed to parse file {}: {}", file_path, e))
        }
    }
}

struct CodeVisitor {
    file_path: String,
    elements: Vec<CodeElement>,
    relationships: Vec<ParseRelationship>,
}

impl CodeVisitor {
    fn new(file_path: String) -> Self {
        Self {
            file_path,
            elements: Vec::new(),
            relationships: Vec::new(),
        }
    }
    
    fn create_element(&self, name: String, element_type: ElementType, line: usize) -> CodeElement {
        CodeElement {
            id: format!("{}:{}:{}", self.file_path, line, name),
            name,
            element_type,
            file_path: self.file_path.clone(),
            start_line: line,
            end_line: line,
        }
    }
}

impl<'ast> Visit<'ast> for CodeVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let name = node.sig.ident.to_string();
        // Use a fallback line number since span().start().line is not available
        let line = 1; // Fallback to line 1
        
        self.elements.push(self.create_element(
            name,
            ElementType::Function,
            line,
        ));
        
        syn::visit::visit_item_fn(self, node);
    }
    
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let name = node.ident.to_string();
        let line = 1; // Fallback to line 1
        
        self.elements.push(self.create_element(
            name,
            ElementType::Struct,
            line,
        ));
        
        syn::visit::visit_item_struct(self, node);
    }
    
    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        let name = node.ident.to_string();
        let line = 1; // Fallback to line 1
        
        self.elements.push(self.create_element(
            name,
            ElementType::Enum,
            line,
        ));
        
        syn::visit::visit_item_enum(self, node);
    }
    
    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        let name = node.ident.to_string();
        let line = 1; // Fallback to line 1
        
        self.elements.push(self.create_element(
            name,
            ElementType::Trait,
            line,
        ));
        
        syn::visit::visit_item_trait(self, node);
    }
    
    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let name = if node.trait_.is_some() {
            format!("impl trait for {}", node.self_ty.to_token_stream())
        } else {
            format!("impl {}", node.self_ty.to_token_stream())
        };
        
        let line = 1; // Fallback to line 1
        
        self.elements.push(self.create_element(
            name,
            ElementType::Impl,
            line,
        ));
        
        syn::visit::visit_item_impl(self, node);
    }
    
    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        let name = node.ident.to_string();
        let line = 1; // Fallback to line 1
        
        self.elements.push(self.create_element(
            name,
            ElementType::Module,
            line,
        ));
        
        syn::visit::visit_item_mod(self, node);
    }
}
