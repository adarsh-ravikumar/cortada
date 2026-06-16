use std::collections::HashMap;

use crate::{
    common::{ERRONEOUS_SPAN, Span},
    context::TypeKind,
};

pub type FunctionId = usize;

#[derive(PartialEq, Eq)]
pub struct FunctionEntry {
    pub decl_span: Span,
    pub symbol_span: Span,

    pub params: Vec<TypeKind>,
    pub arity: usize,

    pub return_type_span: Option<Span>,

    pub return_type: TypeKind,
}

pub static ERRONEOUS_FUNCTION: FunctionEntry = FunctionEntry {
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

impl<'a> FunctionEntry {
    pub fn new(
        decl_span: Span,
        symbol_span: Span,
        params: Vec<TypeKind>,
        return_type_span: Option<Span>,
        return_type: TypeKind,
    ) -> Self {
        Self {
            decl_span,
            symbol_span,
            arity: params.len(),
            params,
            return_type_span,
            return_type,
        }
    }
}
