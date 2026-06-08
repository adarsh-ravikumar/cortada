use std::collections::BTreeSet;

use crate::{
    common::{IOFile, Span},
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    parser::{
        AstNode, AstNodeKind, AtomKind, BinaryExpr, Program, Statements, TypePrimary,
        TypePrimaryKind, TypeUnion, UnaryExpr, VarAssignStatement, VarDeclStatement,
    },
    semantic::{
        AnnotatedStatements, CastAnnotation, VarAssignAnnotation, VarDeclAnnotation,
        annotated_node::{
            AnnotatedTree, AtomAnnotation, BinaryAnnotation, ExpressionAnnotation, FloatAnnotation,
            IntegerAnnotation, StatementAnnotation, UnaryAnnotation,
        },
        operator::{BinaryOpAnnotation, UnaryOpAnnotation},
    },
    symbol_table::{BuiltinType, SymbolTable, TypeKind, UnionType},
};

pub struct SemanticAnalyzer<'a> {
    file: &'a IOFile,
    pub symbol_table: SymbolTable<'a>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(file: &'a IOFile) -> Self {
        Self {
            file,
            symbol_table: SymbolTable::new(file),
        }
    }

    pub fn annotate_program(&mut self, program: Program) -> Result<AnnotatedTree, Diagnostic> {
        // stmts
        let statements = match program.statements.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts)?,
            _ => unreachable!(),
        };

        Ok(AnnotatedTree { statements })
    }

    pub fn annotate_statements(
        &mut self,
        statements: Statements,
    ) -> Result<AnnotatedStatements, Diagnostic> {
        let stmts = statements.stmts;
        let mut annotated: Vec<StatementAnnotation> = Vec::new();

        for stmt in stmts {
            annotated.push(self.annotate_statement(stmt)?);
        }

        Ok(AnnotatedStatements {
            statements: annotated,
        })
    }

    pub fn annotate_statement(
        &mut self,
        statement: Box<AstNode>,
    ) -> Result<StatementAnnotation, Diagnostic> {
        match statement.kind {
            AstNodeKind::Binary(_) | AstNodeKind::Unary(_) => {
                let expr = *self.annotate_expression(statement)?;
                Ok(StatementAnnotation::Expression(expr))
            }

            AstNodeKind::VarDecl(decl) => {
                let decl = self.annotate_var_decl(decl, statement.span)?;
                Ok(StatementAnnotation::VarDecl(decl))
            }

            AstNodeKind::VarAssign(assign) => {
                let assign = self.annotate_var_assign(assign)?;
                Ok(StatementAnnotation::VarAssign(assign))
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, statement.span)?;
                Ok(StatementAnnotation::Expression(ExpressionAnnotation::Atom(
                    atom_annotated,
                )))
            }

            _ => panic!("not implemented"),
        }
    }

    pub fn annotate_type_expression(&mut self, ty: AstNodeKind) -> Result<TypeKind, Diagnostic> {
        match ty {
            AstNodeKind::TypePrimary(primary) => Ok(self.annotate_type_primary(primary)?),
            AstNodeKind::TypeUnion(union) => Ok(self.annotate_type_union(union)?),
            _ => panic!("type of decleration must be a TypePrimary or TypeUnion node"),
        }
    }

    pub fn annotate_type_union(&mut self, union: TypeUnion) -> Result<TypeKind, Diagnostic> {
        let mut variants: BTreeSet<TypeKind> = BTreeSet::new();

        for variant in union.variants {
            match variant.kind {
                AstNodeKind::TypePrimary(ty) => {
                    variants.insert(self.annotate_type_primary(ty)?);
                }

                AstNodeKind::TypeUnion(ty) => {
                    let union = self.annotate_type_union(ty)?;
                    match union {
                        TypeKind::Union(union) => variants.extend(union.variants),
                        _ => panic!(
                            "Annotate type union must return an annotation containing a union"
                        ),
                    };
                }

                _ => panic!("Union can only contain a type primary or a type union node"),
            }
        }

        Ok(TypeKind::Union(UnionType { variants }))
    }

    pub fn annotate_type_primary(&mut self, ty: TypePrimary) -> Result<TypeKind, Diagnostic> {
        match ty.kind {
            TypePrimaryKind::Integer => Ok(TypeKind::Builtin(BuiltinType::Integer)),
            TypePrimaryKind::Float => Ok(TypeKind::Builtin(BuiltinType::Float)),
        }
    }

    pub fn annotate_var_decl(
        &mut self,
        decl: VarDeclStatement,
        decl_span: Span,
    ) -> Result<VarDeclAnnotation, Diagnostic> {
        let symbol_span = decl.name;

        let symbol = self.symbol_table.get_symbol(symbol_span);

        let mut value = self.annotate_expression(decl.value)?;

        let value_type = value.get_type();

        let binding_type: TypeKind;
        let type_span: Option<Span>;

        if let Some(ty) = decl.var_type {
            binding_type = self.annotate_type_expression(ty.kind)?;
            type_span = Some(ty.span);

            if !binding_type.accepts(value_type) {
                if value_type.try_cast(&binding_type) {
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
                            paranthesise: false,
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

        Ok(VarDeclAnnotation {
            entry: id,
            value: *value,
        })
    }

    pub fn annotate_var_assign(
        &mut self,
        assign: VarAssignStatement,
    ) -> Result<VarAssignAnnotation, Diagnostic> {
        let symbol_span = assign.name;

        let symbol = self.symbol_table.get_symbol(symbol_span);

        let mut value = self.annotate_expression(assign.value)?;

        let value_type = value.get_type();

        let binding = match self.symbol_table.get_binding_from_symbol(symbol) {
            Some(b) => b,
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
            if value_type.try_cast(&binding_type) {
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

    pub fn annotate_expression(
        &mut self,
        expression: Box<AstNode>,
    ) -> Result<Box<ExpressionAnnotation>, Diagnostic> {
        let span = expression.span;

        let expr = match expression.kind {
            AstNodeKind::Binary(expr) => {
                ExpressionAnnotation::Binary(self.annotate_binary_expression(expr, span)?)
            }

            AstNodeKind::Unary(expr) => {
                ExpressionAnnotation::Unary(self.annotate_unary_expression(expr, span)?)
            }

            AstNodeKind::Atom(atom) => {
                let atom_annotated = self.annotate_atom(atom, span)?;
                ExpressionAnnotation::Atom(atom_annotated)
            }

            _ => panic!("invalid expression"),
        };

        // do some type checking perhaps

        Ok(Box::new(expr))
    }

    pub fn annotate_cast(
        &self,
        from: TypeKind,
        to: TypeKind,
        expr: Box<ExpressionAnnotation>,
    ) -> Box<ExpressionAnnotation> {
        if from == to {
            return expr;
        }

        Box::new(ExpressionAnnotation::Cast(CastAnnotation {
            from: from.clone(),
            to: to.clone(),
            expr,
        }))
    }

    pub fn annotate_binary_expression(
        &mut self,
        expr: BinaryExpr,
        span: Span,
    ) -> Result<BinaryAnnotation, Diagnostic> {
        let lhs_span = expr.lhs.span;
        let rhs_span = expr.rhs.span;

        let mut lhs = self.annotate_expression(expr.lhs)?;
        let mut rhs = self.annotate_expression(expr.rhs)?;

        let op = BinaryOpAnnotation {
            operator: expr.op,
            span: expr.op_span,
        };

        let lhs_type = lhs.get_type();
        let rhs_type = rhs.get_type();

        let expr_type;

        if let Some(t) = op.get_result_type(lhs_type, rhs_type) {
            expr_type = t;
        }
        // try cast
        else if let Some(res) = op.try_cast(lhs_type, rhs_type) {
            expr_type = res.clone();

            if lhs_type != &res {
                lhs = self.annotate_cast(lhs_type.clone(), res, lhs);
            } else if rhs_type != &res {
                rhs = self.annotate_cast(rhs_type.clone(), res, rhs);
            }
        }
        // bad operation
        else {
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::UnsupportedOperator,

                msg: format!(
                    "cannot apply operator '{}' to values of type '{}' and '{}'",
                    op.operator,
                    lhs_type.display(),
                    rhs_type.display()
                ),

                primary: Label {
                    span: op.span,
                    msg: format!(
                        "'{}' is not defined for '{}' and '{}'",
                        op.operator,
                        lhs_type.display(),
                        rhs_type.display()
                    ),
                    paranthesise: false,
                },

                secondary: vec![
                    Label {
                        span: lhs_span,
                        msg: format!("left-hand side has type '{}'", lhs_type.display(),),
                        paranthesise: true,
                    },
                    Label {
                        span: rhs_span,
                        msg: format!("right-hand side has type '{}'", rhs_type.display(),),
                        paranthesise: true,
                    },
                ],

                notes: vec![],
            });
        }

        Ok(BinaryAnnotation {
            lhs,
            rhs,
            op,
            span,
            expr_type,
        })
    }

    pub fn annotate_unary_expression(
        &mut self,
        expr: UnaryExpr,
        span: Span,
    ) -> Result<UnaryAnnotation, Diagnostic> {
        let operand_span = expr.operand.span;

        let operand = self.annotate_expression(expr.operand)?;
        let op = UnaryOpAnnotation {
            operator: expr.op,
            span: expr.op_span,
        };

        let expr_type;

        let operand_type = operand.get_type();

        if let Some(t) = op.get_result_type(operand_type) {
            expr_type = t;
        }
        // try cast
        else if let Some(t) = op.try_cast(operand_type) {
            // operand = self.annotate_cast(operand_type.clone(), target_type.clone(), operand);
            expr_type = t
        }
        // bad operation
        else {
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::UnsupportedOperator,

                msg: format!(
                    "cannot apply operator '{}' to value of type '{}'",
                    op.operator,
                    operand_type.display(),
                ),

                primary: Label {
                    span: op.span,
                    msg: format!(
                        "'{}' is not defined for '{}'",
                        op.operator,
                        operand_type.display(),
                    ),
                    paranthesise: false,
                },

                secondary: vec![Label {
                    span: operand_span,
                    msg: format!("operand has type '{}'", operand_type.display(),),
                    paranthesise: true,
                }],

                notes: vec![],
            });
        }
        Ok(UnaryAnnotation {
            op,
            operand,
            span,
            expr_type,
        })
    }

    pub fn annotate_atom(
        &mut self,
        atom: AtomKind,
        span: Span,
    ) -> Result<AtomAnnotation, Diagnostic> {
        let atom = match atom {
            AtomKind::Integer(val) => AtomAnnotation::Integer(IntegerAnnotation {
                value: val,
                span,
                atom_type: TypeKind::Builtin(BuiltinType::Integer),
            }),

            AtomKind::Float(val) => AtomAnnotation::Float(FloatAnnotation {
                value: val,
                span,
                atom_type: TypeKind::Builtin(BuiltinType::Integer),
            }),

            _ => panic!("atom visit not implemented"),
        };

        Ok(atom)
    }

    pub fn create_annotated_tree(
        &mut self,
        ast: Box<AstNode>,
    ) -> Result<AnnotatedTree, Diagnostic> {
        let kind = ast.kind;

        match kind {
            AstNodeKind::Program(program) => self.annotate_program(program),
            _ => panic!("Grammar enforces that the topmost node must be a program node"),
        }
    }
}
