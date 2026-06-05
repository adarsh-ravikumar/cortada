use std::collections::{HashMap, hash_map};

use crate::common::Span;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    Integer,
    Float,
    Null,
}

impl SymbolType {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Integer => "int",
            Self::Float => "float",
            Self::Null => "null",
        }
    }
}

pub struct SymbolEntry {
    pub symbol_type: SymbolType,
    pub decl_span: Span,
    pub id_span: Span,
}

pub struct SymbolTable<'a> {
    table: HashMap<&'a str, Vec<SymbolEntry>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        symbol: &'a str,
        symbol_type: SymbolType,
        decl_span: Span,
        id_span: Span,
    ) {
        let entry = SymbolEntry {
            symbol_type,
            decl_span,
            id_span,
        };

        if let Some(existing) = self.table.get_mut(symbol) {
            existing.push(entry);
            return;
        }

        self.table.insert(symbol, vec![entry]);
    }

    pub fn exists(&self, symbol: &'a str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn entry(&self, symbol: &'a str) -> Option<&SymbolEntry> {
        if let Some(entry) = self.table.get(symbol) {
            Some(entry.last().unwrap())
        } else {
            None
        }
    }

    pub fn assign_type(&mut self, symbol: &'a str, symbol_type: SymbolType) {
        if let Some(entry) = self.table.get_mut(symbol) {
            entry.last_mut().unwrap().symbol_type = symbol_type;
        } else {
            panic!("symbol does not exist")
        }
    }

    pub fn iter(&self) -> hash_map::Iter<'_, &str, Vec<SymbolEntry>> {
        self.table.iter()
    }
}
