use std::collections::HashMap;

use crate::{
    common::{IOFile, Span},
    symbol_table::{
        BindingTable, FunctionEntry, FunctionTable, TypeKind,
        binding::{BindingEntry, ERRONEOUS_BINDING},
        function::ERRONEOUS_FUNCTION,
    },
};

pub struct SymbolTable<'a> {
    scopes: Vec<HashMap<&'a str, Vec<usize>>>,
    next_id: usize,
    pub bindings: BindingTable,
    pub functions: FunctionTable,
    file: &'a IOFile,
}

impl<'a> SymbolTable<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            next_id: 1,
            scopes: Vec::new(),
            bindings: BindingTable::new(),
            functions: FunctionTable::new(),
            file,
        }
    }

    pub fn get_symbol(&self, span: Span) -> &'a str {
        self.file.view_span(span)
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() == 1 {
            return;
        }

        self.scopes.pop();
    }

    pub fn add_symbol(&mut self, symbol: &'a str, id: usize) {
        let scope = self.scopes.last_mut().unwrap();

        if let Some(existing) = scope.get_mut(symbol) {
            existing.push(id);
        } else {
            scope.insert(symbol, vec![id]);
        }
    }

    pub fn create_binding(
        &mut self,
        decl_span: Span,
        symbol_span: Span,
        type_span: Option<Span>,
        binding_type: TypeKind,
    ) -> usize {
        let id = self.next_id;

        self.bindings
            .create(id, decl_span, symbol_span, type_span, binding_type);

        self.next_id += 1;

        let symbol = self.get_symbol(symbol_span);

        self.add_symbol(symbol, id);

        id
    }

    pub fn get_binding(&self, symbol: &'a str) -> &BindingEntry {
        let id = self.resolve(symbol);

        if id == 0 {
            return &ERRONEOUS_BINDING;
        }

        self.bindings.get(&id).unwrap()
    }

    pub fn create_function(
        &mut self,
        decl_span: Span,
        symbol_span: Span,
        params: Vec<TypeKind>,
        return_type_span: Option<Span>,
        return_type: TypeKind,
    ) -> usize {
        let id = self.next_id;

        self.functions.create(
            id,
            decl_span,
            symbol_span,
            params,
            return_type_span,
            return_type,
        );

        let symbol = self.get_symbol(symbol_span);

        self.add_symbol(symbol, id);

        id
    }

    pub fn get_function(&self, symbol: &'a str) -> &FunctionEntry {
        let id = self.resolve(symbol);

        if id == 0 {
            return &ERRONEOUS_FUNCTION;
        }

        self.functions.get(&id).unwrap()
    }

    // returns 0 if failure
    pub fn resolve(&self, symbol: &'a str) -> usize {
        for scope in self.scopes.iter().rev() {
            if let Some(scope_entry) = scope.get(symbol) {
                return *scope_entry.last().unwrap();
            }
        }

        0
    }
}
