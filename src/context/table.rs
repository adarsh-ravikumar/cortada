use std::collections::HashMap;

use crate::{
    common::{ERRONEOUS_SPAN, IOFile, Span},
    context::{ContextKind, FunctionEntry, TypeKind, binding::BindingEntry, context::ContextStack},
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
    pub context_stack: ContextStack<'a>,
    table: HashMap<usize, SymbolEntry>,
    next_id: usize,
    file: &'a IOFile,
}

impl<'a> SymbolTable<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            context_stack: ContextStack::new(),
            table: HashMap::new(),
            next_id: 1,
            file,
        }
    }

    pub fn symbol_from_span(&self, span: Span) -> &'a str {
        self.file.view_span(span)
    }

    pub fn get_line_number(&self, span: Span) -> usize {
        self.file.line_from_index(span.start).unwrap() + 1
    }

    pub fn add_symbol(&mut self, symbol: &'a str, id: usize) {
        let context = self.context_stack.get_mut_current();

        if let Some(existing) = context.symbols.get_mut(symbol) {
            existing.push(id);
        } else {
            context.symbols.insert(symbol, vec![id]);
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
    ) -> usize {
        let id = self.next_id;

        match &self.context_stack.get_current().kind {
            ContextKind::Function(ctx) => {
                self.table.insert(
                    id,
                    SymbolEntry {
                        id,
                        kind: SymbolKind::Function(FunctionEntry::new(
                            decl_span,
                            symbol_span,
                            params,
                            ctx.get_return_span(),
                            ctx.return_type.clone(),
                        )),
                    },
                );

                let symbol = self.symbol_from_span(symbol_span);

                self.add_symbol(symbol, id);

                id
            }

            _ => panic!("expected current context to be FunctionContext"),
        }
    }

    pub fn get_symbol(&self, symbol: &'a str) -> &SymbolEntry {
        let id = self.context_stack.resolve_in_parent(symbol);

        if id == 0 {
            return &ERRONEOUS_ENTRY;
        }

        self.table.get(&id).unwrap()
    }

    pub fn get_symbol_history(&self, symbol: &'a str) -> &Vec<usize> {
        self.context_stack
            .get_current()
            .symbols
            .get(symbol)
            .unwrap()
    }

    pub fn get(&self, id: &usize) -> &SymbolEntry {
        self.table.get(id).unwrap()
    }
}
