use std::collections::HashMap;

use crate::{
    common::{ERRONEOUS_SPAN, Span},
    symbol_table::TypeKind,
};

#[derive(PartialEq, Eq)]
pub struct BindingEntry {
    pub id: usize,

    pub decl_span: Span,
    pub symbol_span: Span,
    pub type_span: Option<Span>,

    pub binding_type: TypeKind,
}

pub static ERRONEOUS_BINDING: BindingEntry = BindingEntry {
    id: 0,
    decl_span: ERRONEOUS_SPAN,
    symbol_span: ERRONEOUS_SPAN,
    type_span: None,
    binding_type: TypeKind::Error,
};

pub struct BindingTable {
    table: HashMap<usize, BindingEntry>,
}

impl BindingTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn create(
        &mut self,
        id: usize,
        decl_span: Span,
        symbol_span: Span,
        type_span: Option<Span>,
        binding_type: TypeKind,
    ) {
        self.table.insert(
            id,
            BindingEntry {
                id,
                decl_span,
                symbol_span,
                type_span,
                binding_type,
            },
        );
    }

    pub fn get(&self, id: &usize) -> Option<&BindingEntry> {
        self.table.get(id)
    }

    pub fn get_mut(&mut self, id: &usize) -> Option<&mut BindingEntry> {
        self.table.get_mut(id)
    }
}
