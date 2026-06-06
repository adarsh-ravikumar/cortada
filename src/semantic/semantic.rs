use crate::{
    common::{IOFile, Span},
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label},
    parser::{AstNode, AstNodeKind, VarAssignStatement, VarDeclStatement},
    semantic::{SymbolTable, symbol_table::SymbolType},
};

pub struct SemanticAnalyzer<'a> {
    file: &'a IOFile,
    ast: &'a Box<AstNode>,
    pub table: SymbolTable<'a>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(file: &'a IOFile, ast: &'a Box<AstNode>) -> Self {
        Self {
            file,
            ast,
            table: SymbolTable::new(),
        }
    }

    fn symbol_from_span(&self, span: Span) -> &'a str {
        self.file.view_span(span)
    }

    fn type_from_ident(&self, span: Span) -> Result<SymbolType, Diagnostic> {
        let var_type = self.symbol_from_span(span);

        match var_type {
            "int" => Ok(SymbolType::Integer),
            "float" => Ok(SymbolType::Float),
            "null" => Ok(SymbolType::Null),
            _ => Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::UnknownType,

                msg: format!("unknown type `{}`", var_type),

                primary: Label {
                    span: span,
                    msg: "this type is not defined".into(),
                },

                secondary: vec![],

                notes: vec![],
            }),
        }
    }

    fn visit_expression(&self, expr: &Option<Box<AstNode>>) -> Result<SymbolType, Diagnostic> {
        if expr.is_none() {
            return Ok(SymbolType::Null);
        }

        let expr = expr.as_ref().unwrap();

        let expr_type = match &expr.kind {
            AstNodeKind::Integer(_) => SymbolType::Integer,
            AstNodeKind::Float(_) => SymbolType::Float,
            AstNodeKind::Identifier => self.visit_identifier(expr.span)?,
            _ => unreachable!(),
        };

        Ok(expr_type)
    }

    fn visit_identifier(&self, span: Span) -> Result<SymbolType, Diagnostic> {
        let symbol = self.symbol_from_span(span);

        if let Some(entry) = self.table.entry(symbol) {
            return Ok(entry.symbol_type);
        }

        Err(Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::UndefinedIdentifier,

            msg: format!("identifier `{}` is not defined", symbol),

            primary: Label {
                span: span,
                msg: "unknown identifier".into(),
            },

            secondary: vec![],

            notes: vec!["identifiers must be declared before they can be used".into()],
        })
    }

    fn visit_var_declare(
        &mut self,
        decl: &VarDeclStatement,
        decl_span: Span,
    ) -> Result<(), Diagnostic> {
        println!("{}", decl_span);
        let symbol = self.symbol_from_span(decl.name);

        let expr_type = self.visit_expression(&decl.value)?;

        let symbol_type: SymbolType;

        if let Some(var_type) = decl.var_type {
            symbol_type = self.type_from_ident(var_type)?;

            if expr_type != symbol_type {
                return Err(Diagnostic {
                    severity: DiagnosticSeverity::Error,
                    class: DiagnosticClass::TypeMismatch,

                    msg: format!(
                        "cannot assign value of type `{}` to variable `{}` of type `{}`",
                        expr_type.display(),
                        symbol,
                        symbol_type.display(),
                    ),

                    primary: Label {
                        span: decl.name,
                        msg: format!("this expression has type `{}`", expr_type.display()),
                    },

                    secondary: vec![Label {
                        span: var_type,
                        msg: format!(
                            "`{}` declared with type `{}`",
                            symbol,
                            symbol_type.display()
                        ),
                    }],

                    notes: vec![],
                });
            }
        } else {
            symbol_type = expr_type;
        };

        self.table.insert(symbol, symbol_type, decl_span, decl.name);

        Ok(())
    }

    fn visit_var_assign(&mut self, assign: &VarAssignStatement) -> Result<(), Diagnostic> {
        let symbol = self.symbol_from_span(assign.name);

        let entry = self.table.entry(symbol);

        if entry.is_none() {
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::UndefinedIdentifier,

                msg: format!("cannot assign to undefined identifier `{}`", symbol),

                primary: Label {
                    span: assign.name,
                    msg: "assignment target is not defined".into(),
                },

                secondary: vec![],

                notes: vec!["variables must be declared before they can be assigned to".into()],
            });
        }

        let entry = entry.unwrap();

        // then we visit the value as well
        let expr_type = self.visit_expression(&assign.value)?;

        // this means no type was assigned previously
        // we infer the type here
        if expr_type == SymbolType::Null {
            self.table.assign_type(symbol, expr_type);

            return Ok(());
        }

        // compare types

        if expr_type != entry.symbol_type {
            return Err(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::TypeMismatch,

                msg: format!(
                    "cannot assign value of type `{}` to variable `{}` of type `{}`",
                    expr_type.display(),
                    symbol,
                    entry.symbol_type.display(),
                ),

                primary: Label {
                    span: assign.value_span,
                    msg: format!("this expression has type `{}`", expr_type.display()),
                },

                secondary: vec![Label {
                    span: entry.decl_span,
                    msg: format!(
                        "`{}` was declared here with type `{}`",
                        symbol,
                        entry.symbol_type.display()
                    ),
                }],

                notes: vec![
                    format!("variables retain the type established by their declaration"),
                    format!(
                        "if you intend `{}` to have type `{}` from this point onward, consider shadowing it with a new declaration",
                        symbol,
                        expr_type.display(),
                    ),
                ],
            });
        }

        Ok(())
    }

    fn visit_statement(&mut self, node: &'a Box<AstNode>) -> Result<(), Diagnostic> {
        match &node.kind {
            AstNodeKind::VarDecl(decl) => self.visit_var_declare(decl, node.span)?,
            AstNodeKind::VarAssign(assign) => self.visit_var_assign(assign)?,
            AstNodeKind::Identifier => {
                let _ = self.visit_identifier(node.span)?;
            }
            _ => unreachable!(),
        };

        Ok(())
    }

    fn visit_node(&mut self, node: &'a Box<AstNode>) -> Result<(), Diagnostic> {
        match &node.kind {
            AstNodeKind::Statements(statements) => {
                for stmt in statements.stmts.iter() {
                    self.visit_statement(stmt)?;
                }
            }

            kind => println!("Visit for {kind:?} not implemented!"),
        }

        Ok(())
    }

    pub fn build_table(&mut self) -> Result<(), Diagnostic> {
        self.visit_node(self.ast)
    }
}
