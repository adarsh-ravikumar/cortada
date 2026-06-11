use std::collections::HashMap;

use crate::{
    common::{ERRONEOUS_SPAN, IOFile, Span},
    symbol_table::{FunctionEntry, TypeKind, binding::BindingEntry},
};

#[derive(PartialEq, Eq)]
pub enum SymbolKind {
    Binding(BindingEntry),
    Function(FunctionEntry),
    Erroneous,
}

pub struct SymbolEntry {
    pub id: usize,
    pub kind: SymbolKind,
}

impl SymbolEntry {
    pub fn get_decl_span(&self) -> Span {
        match &self.kind {
            SymbolKind::Binding(binding) => binding.decl_span,
            SymbolKind::Function(function) => function.decl_span,
            SymbolKind::Erroneous => ERRONEOUS_SPAN,
        }
    }

    pub fn get_symbol_span(&self) -> Span {
        match &self.kind {
            SymbolKind::Binding(binding) => binding.symbol_span,
            SymbolKind::Function(function) => function.symbol_span,
            SymbolKind::Erroneous => ERRONEOUS_SPAN,
        }
    }

    pub fn get_type(&self) -> &TypeKind {
        match &self.kind {
            SymbolKind::Binding(binding) => &binding.binding_type,
            SymbolKind::Function(function) => &function.return_type,
            SymbolKind::Erroneous => &TypeKind::Error,
        }
    }
}

pub static ERRONEOUS_ENTRY: SymbolEntry = SymbolEntry {
    id: 0,
    kind: SymbolKind::Erroneous,
};

pub struct SymbolTable<'a> {
    scopes: Vec<HashMap<&'a str, Vec<usize>>>,
    pub stack_top: usize,
    next_id: usize,
    table: HashMap<usize, SymbolEntry>,
    file: &'a IOFile,
}

impl<'a> SymbolTable<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            next_id: 1,
            stack_top: 0,
            scopes: Vec::new(),
            table: HashMap::new(),
            file,
        }
    }

    pub fn symbol_from_span(&self, span: Span) -> &'a str {
        self.file.view_span(span)
    }

    pub fn get_line_number(&self, span: Span) -> usize {
        self.file.line_from_index(span.start).unwrap() + 1
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.stack_top = self.scopes.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        if self.stack_top == 0 {
            return;
        }

        self.stack_top -= 1
    }

    pub fn add_symbol(&mut self, symbol: &'a str, id: usize) {
        let scope = self.scopes.get_mut(self.stack_top).unwrap();
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

        self.table.insert(
            id,
            SymbolEntry {
                id,
                kind: SymbolKind::Binding(BindingEntry::new(
                    decl_span,
                    symbol_span,
                    type_span,
                    binding_type,
                )),
            },
        );

        self.next_id += 1;

        let symbol = self.symbol_from_span(symbol_span);

        self.add_symbol(symbol, id);

        id
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

        self.table.insert(
            id,
            SymbolEntry {
                id,
                kind: SymbolKind::Function(FunctionEntry::new(
                    decl_span,
                    symbol_span,
                    params,
                    return_type_span,
                    return_type,
                )),
            },
        );

        let symbol = self.symbol_from_span(symbol_span);

        self.add_symbol(symbol, id);

        id
    }

    pub fn get_symbol(&self, symbol: &'a str) -> &SymbolEntry {
        let id = self.resolve_in_parent(symbol);

        if id == 0 {
            return &ERRONEOUS_ENTRY;
        }

        self.table.get(&id).unwrap()
    }

    pub fn get_symbol_history(&self, symbol: &'a str) -> &Vec<usize> {
        self.scopes
            .get(self.stack_top)
            .unwrap()
            .get(symbol)
            .unwrap()
    }

    pub fn get(&self, id: &usize) -> &SymbolEntry {
        self.table.get(id).unwrap()
    }

    // returns 0 if failure
    pub fn resolve_in_parent(&self, symbol: &'a str) -> usize {
        let mut top = self.stack_top;

        loop {
            let scope = self.scopes.get(top).unwrap();

            if let Some(scope_entry) = scope.get(symbol) {
                return *scope_entry.last().unwrap();
            }

            if top > 0 {
                top -= 1;
            }

            if top == 0 {
                break;
            }
        }

        0
    }

    // returns 0 if failure
    pub fn resolve_in_child(&self, symbol: &'a str) -> usize {
        let mut bottom = self.stack_top;

        while bottom < self.scopes.len() {
            let scope = self.scopes.get(bottom).unwrap();

            if let Some(scope_entry) = scope.get(symbol) {
                return *scope_entry.last().unwrap();
            }

            bottom += 1;
        }

        0
    }
}
