use crate::{
    common::{ERRONEOUS_SPAN, Span},
    context::{ContextKind, TypeKind},
    parser::{AstNodeKind, FnStatement},
    semantic::{ExpressionAnnotation, FunctionAnnotation, SemanticAnalyzer, VarDeclAnnotation},
};

impl<'a, 'scope> SemanticAnalyzer<'a> {
    pub fn annotate_function(
        &mut self,
        fn_stmt: FnStatement,
        decl_span: Span,
    ) -> FunctionAnnotation {
        let mut params: Vec<VarDeclAnnotation> = Vec::new();
        let mut param_types: Vec<TypeKind> = Vec::new();

        self.symbol_table
            .context_stack
            .enter_context(ContextKind::function_context());

        for param in fn_stmt.params {
            match param.kind {
                AstNodeKind::VarDecl(decl) => {
                    let mut param = self.annotate_var_decl(decl, param.span);

                    param.value = ExpressionAnnotation::Null;

                    let param_type = if param.entry != 0 {
                        self.symbol_table.get(&param.entry).get_type().clone()
                    } else {
                        TypeKind::Error
                    };

                    params.push(param);
                    param_types.push(param_type);
                }
                _ => panic!("params can only be var decl statements"),
            };
        }

        if let Some(ty) = fn_stmt.return_type {
            let span = ty.span;
            let ty = self.annotate_type_expression(ty.kind);

            self.symbol_table
                .context_stack
                .try_set_context_return(ty, span, ERRONEOUS_SPAN, false);
        }

        let body = match fn_stmt.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => panic!("body must be statements"),
        };

        let entry = self
            .symbol_table
            .create_function(decl_span, fn_stmt.name, param_types);

        self.symbol_table.context_stack.exit_context();

        FunctionAnnotation {
            entry,
            params,
            body,
        }
    }
}
