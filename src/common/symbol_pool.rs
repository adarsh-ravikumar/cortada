use std::collections::HashMap;

use crate::common::{IOFile, ScopeTable, Span, Type};

pub struct BindingEntry {
    symbol_span: Span,
    decl_span: Span,

    binding_type: Type,
}

pub enum SymbolType {
    // Binding[] will have atleast one element
    // the last entry is the current entry
    // the entire list maintains a history of shadowing
    Binding(Vec<BindingEntry>),
}

pub struct SymbolEntry {
    id: usize,
    symbol_entry: SymbolType,
}

impl SymbolType {
    pub fn binding(symbol_span: Span, decl_span: Span, binding_type: Type) -> BindingEntry {
        BindingEntry {
            symbol_span,
            decl_span,
            binding_type: binding_type,
        }
    }
}

pub struct SymbolPool<'a> {
    file: &'a IOFile,

    pool: HashMap<usize, SymbolEntry>,
    next_id: usize,

    scope_table: ScopeTable<'a>,
    current_scope: usize,
}

impl<'a> SymbolPool<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            file,

            pool: HashMap::new(),
            next_id: 0,

            scope_table: ScopeTable::new(),
            current_scope: 0,
        }
    }

    fn get_symbol(&self, span: Span) -> &'a str {
        self.file.view_span(span)
    }

    pub fn create_binding(&mut self, symbol_entry: BindingEntry) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let symbol = self.get_symbol(symbol_entry.symbol_span);

        // if binding exists in current scope, shadow it
        if let Some(id) = self.scope_table.symbol_id(self.current_scope, symbol) {
            let existing = self.pool.get_mut(id).unwrap();

            match &mut existing.symbol_entry {
                SymbolType::Binding(history) => history.push(symbol_entry),
                // _ => unreachable!(),
            }

            return *id;
        }

        // else create it
        let entry = SymbolEntry {
            id,
            symbol_entry: SymbolType::Binding(vec![symbol_entry]),
        };

        self.pool.insert(id, entry);

        id
    }
}
