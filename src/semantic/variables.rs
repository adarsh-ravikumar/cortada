use crate::diagnostic::LabelKind;
use crate::semantic::{ExpressionAnnotation, SemanticAnalyzer};

use crate::symbol_table::{BindingEntry, ScopeEntryKind, ScopeTable};
use crate::{
    common::Span,
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    parser::{VarAssignStatement, VarDeclStatement},
    semantic::{VarAssignAnnotation, VarDeclAnnotation},
    symbol_table::TypeKind,
};

impl<'a, 'scope> SemanticAnalyzer<'a> {
    pub fn annotate_var_decl(
        &mut self,
        decl: VarDeclStatement,
        decl_span: Span,
        scope: &mut ScopeTable<'a, 'scope>,
    ) -> VarDeclAnnotation {
        let symbol_span = decl.name;

        let symbol = self.symbol_table.get_symbol(symbol_span);

        let binding_type: TypeKind;
        let type_span: Option<Span>;
        let mut value: Box<ExpressionAnnotation>;
        let value_span: Span;

        // if there is a type expression, then we have to optionally look for a value
        if let Some(ty) = decl.var_type {
            binding_type = self.annotate_type_expression(ty.kind);
            type_span = Some(ty.span);

            if let Some(val) = decl.value {
                value = self.annotate_expression(val, scope);
                value_span = decl.value_span.unwrap();
            } else {
                value = Box::new(ExpressionAnnotation::Null);
                value_span = Span::new(0, 0);
            }
        }
        // if there is no type expression, then we have to look for the value and infer type
        else {
            type_span = None;
            value = self.annotate_expression(decl.value.unwrap(), scope);
            binding_type = value.get_type().clone();
            value_span = decl.value_span.unwrap();
        }

        let value_type: &TypeKind = value.get_type();

        if !binding_type.accepts(value_type) {
            if value_type.try_implicit_cast(&binding_type) {
                value = self.annotate_cast(value_type.clone(), binding_type.clone(), value);
            } else {
                self.diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::TypeMismatch,

                    msg: format!(
                        "cannot assign value of type `{}` to variable `{}` of type `{}`",
                        value_type.display(),
                        symbol,
                        binding_type.display(),
                    ),

                    location: value_span,
                    labels: vec![
                        Label {
                            span: value_span,
                            msg: format!("this expression has type `{}`", value_type.display()),
                            paranthesise: true,
                            kind: LabelKind::Primary,
                        },
                        Label {
                            span: type_span.unwrap(),
                            msg: format!(
                                "`{}` declared with type `{}`",
                                symbol,
                                binding_type.display()
                            ),
                            paranthesise: true,
                            kind: LabelKind::Secondary,
                        },
                    ],

                    notes: vec![],
                });
            }
        }

        let id = self
            .symbol_table
            .create_binding(decl_span, symbol_span, type_span, binding_type);

        scope.add_symbol(symbol, ScopeEntryKind::Binding, id);

        VarDeclAnnotation {
            entry: id,
            value: *value,
        }
    }

    pub fn annotate_var_assign(
        &mut self,
        assign: VarAssignStatement,
        scope: &ScopeTable,
    ) -> VarAssignAnnotation {
        let symbol_span = assign.name;

        let symbol = self.symbol_table.get_symbol(symbol_span);

        let mut value = self.annotate_expression(assign.value, scope);

        let value_type = value.get_type();

        let binding = match scope.get_id(symbol) {
            Some(id) => self.symbol_table.get_binding(id).unwrap(),
            None => {
                self.diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::UndefinedIdentifier,

                    msg: format!("cannot assign to undefined identifier `{}`", symbol),

                    location: assign.name,
                    labels: vec![Label {
                        span: assign.name,
                        msg: "assignment target is not defined".into(),
                        paranthesise: false,
                        kind: LabelKind::Primary,
                    }],

                    notes: vec!["variables must be declared before they can be assigned to".into()],
                });

                &BindingEntry::ERRONEOUS
            }
        };

        let binding_type = &binding.binding_type;

        if !binding_type.accepts(value_type) {
            if value_type.try_implicit_cast(&binding_type) {
                value = self.annotate_cast(value_type.clone(), binding_type.clone(), value);
            } else {
                self.diagnostics.push(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::TypeMismatch,

                    msg: format!(
                        "cannot assign a value of type `{}` to variable `{}` of type `{}`",
                        value_type.display(),
                        symbol,
                        binding_type.display(),
                    ),

                    location: assign.value_span,
                    labels: vec![
                        Label {
                            span: assign.value_span,
                            msg: format!("this expression has type `{}`", value_type.display()),
                            paranthesise: true,
                            kind: LabelKind::Primary,
                        },
                        Label {
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
                            kind: LabelKind::Secondary,
                        },
                    ],

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

        VarAssignAnnotation {
            entry_reference: binding.id,
            value: *value,
        }
    }
}
