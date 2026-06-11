use crate::{
    common::Span,
    parser::{AstNodeKind, FnStatement},
    semantic::{ExpressionAnnotation, FunctionAnnotation, SemanticAnalyzer, VarDeclAnnotation},
    symbol_table::TypeKind,
};

impl<'a, 'scope> SemanticAnalyzer<'a> {
    pub fn annotate_function(
        &mut self,
        fn_stmt: FnStatement,
        decl_span: Span,
    ) -> FunctionAnnotation {
        let mut params: Vec<VarDeclAnnotation> = Vec::new();
        let mut param_types: Vec<TypeKind> = Vec::new();

        self.symbol_table.enter_scope();

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

        let mut return_type: TypeKind = TypeKind::Error; // this will be changed once control-flow
        // analysis is implemented
        let mut return_type_span: Option<Span> = None;

        if let Some(ty) = fn_stmt.return_type {
            return_type_span = Some(ty.span);
            return_type = self.annotate_type_expression(ty.kind);
        }
        // for now, just annotate the body

        let body = match fn_stmt.body.kind {
            AstNodeKind::Statements(stmts) => self.annotate_statements(stmts),
            _ => panic!("body must be statements"),
        };

        let entry = self.symbol_table.create_function(
            decl_span,
            fn_stmt.name,
            param_types,
            return_type_span,
            return_type,
        );

        self.symbol_table.exit_scope();

        FunctionAnnotation {
            entry,
            params,
            body,
        }
    }
}
