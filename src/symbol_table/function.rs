use std::collections::HashMap;

use crate::{
    common::{ERRONEOUS_SPAN, Span},
    symbol_table::TypeKind,
};

pub type FunctionId = usize;

pub struct FunctionEntry {
    pub id: FunctionId,

    pub decl_span: Span,
    pub symbol_span: Span,

    pub params: Vec<TypeKind>,
    pub arity: usize,

    pub return_type_span: Option<Span>,

    pub return_type: TypeKind,
}

pub static ERRONEOUS_FUNCTION: FunctionEntry = FunctionEntry {
    id: 0,
    decl_span: ERRONEOUS_SPAN,
    symbol_span: ERRONEOUS_SPAN,
    params: Vec::new(),
    arity: 0,
    return_type_span: None,
    return_type: TypeKind::Error,
};

pub struct FunctionTable {
    table: HashMap<FunctionId, FunctionEntry>,
}

impl<'a> FunctionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn create(
        &mut self,
        id: FunctionId,
        decl_span: Span,
        symbol_span: Span,
        params: Vec<TypeKind>,
        return_type_span: Option<Span>,
        return_type: TypeKind,
    ) {
        self.table.insert(
            id,
            FunctionEntry {
                id,
                decl_span,
                symbol_span,
                arity: params.len(),
                params,
                return_type_span,
                return_type,
            },
        );
    }

    pub fn get(&self, id: &FunctionId) -> Option<&FunctionEntry> {
        self.table.get(id)
    }

    pub fn get_mut(&mut self, id: &FunctionId) -> Option<&mut FunctionEntry> {
        self.table.get_mut(id)
    }
}
