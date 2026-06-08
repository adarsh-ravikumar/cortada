use crate::semantic::SemanticAnalyzer;

use crate::symbol_table::ScopeTable;
use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    parser::{VarAssignStatement, VarDeclStatement},
    semantic::{VarAssignAnnotation, VarDeclAnnotation},
    symbol_table::TypeKind,
};

impl<'a> SemanticAnalyzer<'a> {
    pub fn annotate_var_decl(
        &mut self,
        decl: VarDeclStatement,
        decl_span: Span,
        scope: &mut ScopeTable<'a>,
    ) -> Result<VarDeclAnnotation, Diagnostic> {
        let symbol_span = decl.name;

        let symbol = self.symbol_table.get_symbol(symbol_span);

        let mut value = self.annotate_expression(decl.value, scope)?;

        let value_type = value.get_type();

        let binding_type: TypeKind;
        let type_span: Option<Span>;

        if let Some(ty) = decl.var_type {
            binding_type = self.annotate_type_expression(ty.kind)?;
            type_span = Some(ty.span);

            if !binding_type.accepts(value_type) {
                if value_type.try_implicit_cast(&binding_type) {
                    value = self.annotate_cast(value_type.clone(), binding_type.clone(), value);
                } else {
                    return Err(Diagnostic {
                        severity: DiagnosticSeverity::Error,
                        class: DiagnosticClass::TypeMismatch,

                        msg: format!(
                            "cannot assign value of type `{}` to variable `{}` of type `{}`",
                            value_type.display(),
                            symbol,
                            binding_type.display(),
                        ),

                        primary: Label {
                            span: decl.value_span,
                            msg: format!("this expression has type `{}`", value_type.display()),
                            paranthesise: true,
                        },

                        secondary: vec![Label {
                            span: ty.span,
                            msg: format!(
                                "`{}` declared with type `{}`",
                                symbol,
                                binding_type.display()
                            ),
                            paranthesise: true,
                        }],

                        notes: vec![],
                    });
                }
            }
        } else {
            binding_type = value_type.clone();
            type_span = None
        };

        let id = self
            .symbol_table
            .create_binding(decl_span, symbol_span, type_span, binding_type);

        scope.add_symbol(symbol, id);

        Ok(VarDeclAnnotation {
            entry: id,
            value: *value,
        })
    }

    pub fn annotate_var_assign(
        &mut self,
        assign: VarAssignStatement,
        scope: &ScopeTable,
    ) -> Result<VarAssignAnnotation, Diagnostic> {
        let symbol_span = assign.name;

        let symbol = self.symbol_table.get_symbol(symbol_span);

        let mut value = self.annotate_expression(assign.value, scope)?;

        let value_type = value.get_type();

        let binding = match scope.get_id(symbol) {
            Some(id) => self.symbol_table.get_binding(id).unwrap(),
            None => {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::UndefinedIdentifier,

                    msg: format!("cannot assign to undefined identifier `{}`", symbol),

                    primary: Label {
                        span: assign.name,
                        msg: "assignment target is not defined".into(),
                        paranthesise: false,
                    },

                    secondary: vec![],

                    notes: vec!["variables must be declared before they can be assigned to".into()],
                });
            }
        };

        let binding_type = &binding.binding_type;

        if !binding_type.accepts(value_type) {
            if value_type.try_implicit_cast(&binding_type) {
                value = self.annotate_cast(value_type.clone(), binding_type.clone(), value);
            } else {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::TypeMismatch,

                    msg: format!(
                        "cannot assign a value of type `{}` to variable `{}` of type `{}`",
                        value_type.display(),
                        symbol,
                        binding_type.display(),
                    ),

                    primary: Label {
                        span: assign.value_span,
                        msg: format!("this expression has type `{}`", value_type.display()),
                        paranthesise: true,
                    },

                    secondary: vec![Label {
                        span: if let Some(span) = binding.type_span {
                            span
                        } else {
                            binding.decl_span
                        },
                        msg: format!(
                            "`{}` declared with type `{}`",
                            symbol,
                            binding_type.display()
                        ),
                        paranthesise: binding.type_span.is_some(),
                    }],

                    notes: vec![
                        format!("variables retain the type established by their declaration"),
                        format!(
                            "if you intend `{}` to have type `{}` from this point onward, consider shadowing it with a new declaration",
                            symbol,
                            value_type.display(),
                        ),
                    ],
                });
            }
        }

        Ok(VarAssignAnnotation {
            entry_reference: binding.id,
            value: *value,
        })
    }
}
