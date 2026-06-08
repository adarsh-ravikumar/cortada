use std::collections::HashMap;

use crate::{common::Span, symbol_table::TypeKind};

pub type BindingId = usize;

pub struct BindingEntry {
    pub id: BindingId,

    pub decl_span: Span,
    pub symbol_span: Span,
    pub type_span: Option<Span>,

    pub binding_type: TypeKind,
}

pub struct BindingTable {
    table: HashMap<BindingId, BindingEntry>,
}

impl BindingTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn create(
        &mut self,
        id: BindingId,
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

    pub fn get(&self, id: &BindingId) -> Option<&BindingEntry> {
        self.table.get(id)
    }

    pub fn get_mut(&mut self, id: &BindingId) -> Option<&mut BindingEntry> {
        self.table.get_mut(id)
    }
}
