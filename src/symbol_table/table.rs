use crate::{
    common::{IOFile, Span},
    symbol_table::{
        BindingId, BindingTable, FunctionEntry, FunctionTable, TypeKind, binding::BindingEntry,
    },
};

pub struct SymbolTable<'a> {
    source: &'a IOFile,

    binding_table: BindingTable,
    function_table: FunctionTable,

    next_id: usize,
}

impl<'a> SymbolTable<'a> {
    // ID starts with 1
    // ID of 0 is erroneous
    pub fn new(source: &'a IOFile) -> Self {
        Self {
            source,
            binding_table: BindingTable::new(),
            function_table: FunctionTable::new(),
            next_id: 1,
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

        self.next_id += 1;

        id
    }

    pub fn get_binding(&self, id: &usize) -> Option<&BindingEntry> {
        self.binding_table.get(id)
    }

    pub fn create_function(
        &mut self,
        decl_span: Span,
        symbol_span: Span,
        params: Vec<TypeKind>,
        return_type_span: Option<Span>,
        return_type: TypeKind,
    ) -> BindingId {
        let id = self.next_id;

        self.function_table.create(
            id,
            decl_span,
            symbol_span,
            params,
            return_type_span,
            return_type,
        );

        self.next_id += 1;

        id
    }

    pub fn get_function(&self, id: &usize) -> Option<&FunctionEntry> {
        self.function_table.get(id)
    }
}
