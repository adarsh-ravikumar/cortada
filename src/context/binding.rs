use crate::{
    common::{ERRONEOUS_SPAN, Span},
    context::TypeKind,
};

#[derive(PartialEq, Eq)]
pub struct BindingEntry {
    pub decl_span: Span,
    pub symbol_span: Span,
    pub type_span: Option<Span>,

    pub binding_type: TypeKind,
}

pub static ERRONEOUS_BINDING: BindingEntry = BindingEntry {
    decl_span: ERRONEOUS_SPAN,
    symbol_span: ERRONEOUS_SPAN,
    type_span: None,
    binding_type: TypeKind::Error,
};

impl BindingEntry {
    pub fn new(
        decl_span: Span,
        symbol_span: Span,
        type_span: Option<Span>,
        binding_type: TypeKind,
    ) -> Self {
        Self {
            decl_span,
            symbol_span,
            type_span,
            binding_type,
        }
    }
}
