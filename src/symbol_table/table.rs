use crate::{
    common::{IOFile, Span},
    symbol_table::{BindingId, BindingTable, TypeKind, binding::BindingEntry, scope::ScopeTable},
};

pub struct SymbolTable<'a> {
    source: &'a IOFile,

    binding_table: BindingTable,
    next_id: usize,

    scope_stack: Vec<ScopeTable<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new(source: &'a IOFile) -> Self {
        Self {
            source,
            binding_table: BindingTable::new(),
            next_id: 0,
            scope_stack: vec![ScopeTable::new()],
        }
    }

    pub fn get_symbol(&self, span: Span) -> &'a str {
        self.source.view_span(span)
    }

    pub fn create_binding(
        &mut self,
        decl_span: Span,
        symbol_span: Span,
        type_span: Option<Span>,
        binding_type: TypeKind,
    ) -> BindingId {
        let id = self.next_id;

        self.binding_table
            .create(id, decl_span, symbol_span, type_span, binding_type);

        let name = self.get_symbol(symbol_span);
        self.scope_stack.last_mut().unwrap().add_symbol(name, id);

        self.next_id += 1;

        id
    }

    pub fn get_binding_from_symbol(&self, symbol: &'a str) -> Option<&BindingEntry> {
        let scope = self.scope_stack.last().unwrap();
        let id = match scope.get_id(symbol) {
            Some(id) => id,
            None => return None,
        };

        self.binding_table.get(id)
    }

    pub fn get_binding_from_id(&self, id: &usize) -> Option<&BindingEntry> {
        self.binding_table.get(id)
    }
}
