use thiserror::Error;

use std::path::Path;
use std::collections::HashSet;

pub fn parse_file_from_path(path: &Path) -> Result<syn::File, DaoError> {
    let file_string = std::fs::read_to_string(path)?;
    let token_stream = syn::parse_file(&file_string)?;
    Ok(token_stream)
}

pub fn filter_attributes(attrs: &[&syn::Attribute]) -> HashSet<String> {
    let mut filtered = HashSet::new();

    for attr in attrs {
        if attr.path.is_ident("cfg") {
            if let Ok(meta) = attr.parse_meta() {
                recurse_filter_meta(meta, &mut filtered);
            }
        }
    }

    filtered
}

fn recurse_filter_meta(meta: syn::Meta, filtered_set: &mut HashSet<String>) {
    match meta {
        syn::Meta::List(list) => {
            for child in list.nested {
                if let syn::NestedMeta::Meta(child_meta) = child {
                    recurse_filter_meta(child_meta, filtered_set);
                }
            }
        },
        syn::Meta::NameValue(name_value) => {
            if name_value.path.is_ident("dao") {
                if let syn::Lit::Str(str) = name_value.lit {
                    filtered_set.insert(str.value());
                }
            }
        },
        _ => {},
    }
}

#[derive(Default)]
pub struct AttributeCollector<'a> {
    pub attributes: Vec<&'a syn::Attribute>,
}

impl<'a> AttributeCollector<'a> {
    pub fn collect_attributes(&mut self, file: &'a syn::File) {
        syn::visit::visit_file(self, file);
    }    
}

impl<'a> syn::visit::Visit<'a> for AttributeCollector<'a> {
    fn visit_attribute(&mut self, attr: &'a syn::Attribute) {
        self.attributes.push(attr);
    }
}

#[derive(Error, Debug)]
pub enum DaoError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Error parsing rust code: {0}")]
    SyntaxError(#[from] syn::Error),
}
