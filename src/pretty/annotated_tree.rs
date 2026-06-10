use crate::{
    parser::AstNodeKind,
    semantic::{
        AnnotatedStatements, AnnotatedTree, AtomAnnotation, BinaryAnnotation, BoolAnnotation,
        CastAnnotation, ExpressionAnnotation, FloatAnnotation, FunctionAnnotation,
        IdentifierAnnotation, IfAnnotation, IntegerAnnotation, StatementAnnotation,
        UnaryAnnotation, VarAssignAnnotation, VarDeclAnnotation, WhileAnnotation,
    },
    symbol_table::SymbolTable,
    utils::Style,
};

pub struct AnnotatedTreePrinter<'a> {
    pub symbol_table: &'a SymbolTable<'a>,
}

impl<'a> AnnotatedTreePrinter<'a> {
    pub fn new(symbol_table: &'a SymbolTable<'a>) -> Self {
        Self { symbol_table }
    }

    fn generate_space_leader(level: usize) -> String {
        "│   ".repeat(level - 1)
    }

    fn generate_field_leader(level: usize, is_terminal: bool) -> String {
        if level > 0 {
            format!(
                "{}{}{} {}",
                Style::BRIGHT_BLACK,
                Self::generate_space_leader(level),
                if is_terminal {
                    "└──"
                } else {
                    "├──"
                },
                Style::RESET
            )
        } else {
            "".to_string()
        }
    }

    fn is_terminal(kind: &AstNodeKind) -> bool {
        matches!(kind, AstNodeKind::Atom(_))
    }

    fn print_statement(&self, stmt: &StatementAnnotation, level: usize, is_terminal: bool) {
        match stmt {
            StatementAnnotation::Expression(expr) => {
                self.print_expression(expr, level, is_terminal)
            }

            StatementAnnotation::VarDecl(decl) => self.print_var_decl(decl, level, is_terminal),

            StatementAnnotation::VarAssign(assign) => {
                self.print_var_assign(assign, level, is_terminal)
            }

            StatementAnnotation::If(if_stmt) => self.print_if(if_stmt, level, is_terminal),

            StatementAnnotation::While(while_stmt) => {
                self.print_while(while_stmt, level, is_terminal)
            }

            StatementAnnotation::Fn(fn_stmt) => self.print_fn(fn_stmt, level, is_terminal),
        }
    }

    fn print_var_decl(&self, decl: &VarDeclAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

        println!(
            "{leader}{}VarDecl{} {}[id: {}]{}",
            Style::BLUE,
            Style::RESET,
            Style::DIM,
            decl.entry,
            Style::RESET_DIM
        );

        let leader = Self::generate_field_leader(level + 1, false);

        let entry = self.symbol_table.get_binding(&decl.entry).unwrap();

        let name = self.symbol_table.get_symbol(entry.symbol_span);

        println!(
            "{leader}{}name: {}{}{}",
            Style::BLUE,
            Style::BRIGHT_YELLOW,
            name,
            Style::RESET
        );

        println!(
            "{leader}{}type: {}{}",
            Style::BLUE,
            Style::RESET,
            entry.binding_type.display(),
        );

        println!("{leader}{}value{}", Style::BLUE, Style::RESET,);
        self.print_expression(&decl.value, level + 2, is_terminal);
    }

    fn print_var_assign(&self, assign: &VarAssignAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

        println!(
            "{leader}{}VarAssign{} {}[id: {}]{}",
            Style::BLUE,
            Style::RESET,
            Style::DIM,
            assign.entry_reference,
            Style::RESET_DIM
        );

        let leader = Self::generate_field_leader(level + 1, false);

        let entry = self
            .symbol_table
            .get_binding(&assign.entry_reference)
            .unwrap();

        let name = self.symbol_table.get_symbol(entry.symbol_span);

        println!(
            "{leader}{}name: {}{}{}",
            Style::BLUE,
            Style::BRIGHT_YELLOW,
            name,
            Style::RESET
        );

        println!("{leader}{}value{}", Style::BLUE, Style::RESET,);
        self.print_expression(&assign.value, level + 2, is_terminal);
    }

    fn print_fn(&self, stmt: &FunctionAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

        let entry = self.symbol_table.get_function(&stmt.entry).unwrap();

        println!(
            "{leader}{}{}Function{}{}",
            Style::BOLD,
            Style::MAGENTA,
            Style::RESET,
            Style::RESET_BOLD
        );

        println!(
            "{}{}{}Return type: {}{}",
            Style::BRIGHT_BLACK,
            Self::generate_field_leader(level + 1, false),
            Style::MAGENTA,
            Style::RESET,
            entry.return_type.display()
        );

        println!(
            "{}{}{}Params",
            Style::BRIGHT_BLACK,
            Self::generate_field_leader(level + 1, false),
            Style::MAGENTA,
        );

        for param in stmt.params.iter() {
            self.print_var_decl(param, level + 2, false);
        }

        println!(
            "{}{}{}Body",
            Style::BRIGHT_BLACK,
            Self::generate_field_leader(level + 1, false),
            Style::MAGENTA,
        );

        self.print_statements(&stmt.body, level + 2, is_terminal);
    }

    fn print_if(&self, stmt: &IfAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

        println!(
            "{leader}{}{}If{}{}",
            Style::BOLD,
            Style::MAGENTA,
            Style::RESET,
            Style::RESET_BOLD
        );

        // condition
        println!(
            "{}{}{}condition",
            Style::BRIGHT_BLACK,
            Self::generate_field_leader(level + 1, false),
            Style::MAGENTA,
        );

        self.print_expression(&stmt.condition, level + 2, false);

        // body
        self.print_statements(&stmt.body, level + 1, false);

        // elif
        let tree_ch = if stmt.else_stmt.is_some() {
            "├──"
        } else {
            "└──"
        };

        if !stmt.elif_stmts.is_empty() {
            println!(
                "{}{}{tree_ch} {}{}Elif{}{}",
                Style::BRIGHT_BLACK,
                Self::generate_field_leader(level + 1, false),
                Style::BOLD,
                Style::MAGENTA,
                Style::RESET,
                Style::RESET_BOLD
            );
        }

        for elif_stmt in stmt.elif_stmts.iter() {
            // condition
            println!(
                "{}{}├── {}condition",
                Style::BRIGHT_BLACK,
                Self::generate_field_leader(level + 2, false),
                Style::MAGENTA,
            );

            self.print_expression(&elif_stmt.condition, level + 3, false);

            self.print_statements(&elif_stmt.body, level + 2, false);
        }

        // else
        if stmt.else_stmt.is_some() {
            println!(
                "{}{}└── {}{}Else{}{}",
                Style::BRIGHT_BLACK,
                Self::generate_field_leader(level + 1, is_terminal),
                Style::BOLD,
                Style::MAGENTA,
                Style::RESET,
                Style::RESET_BOLD
            );

            self.print_statements(&stmt.else_stmt.as_ref().unwrap(), level + 2, false);
        }
    }

    fn print_while(&self, stmt: &WhileAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

        println!(
            "{leader}{}{}While{}{}",
            Style::BOLD,
            Style::MAGENTA,
            Style::RESET,
            Style::RESET_BOLD
        );

        // condition
        println!(
            "{}{}{}condition",
            Style::BRIGHT_BLACK,
            Self::generate_field_leader(level + 1, false),
            Style::MAGENTA,
        );

        self.print_expression(&stmt.condition, level + 2, false);

        // body
        self.print_statements(&stmt.body, level + 1, false);

        // else
        if stmt.else_stmt.is_some() {
            println!(
                "{}{}└── {}{}Else{}{}",
                Style::BRIGHT_BLACK,
                Self::generate_field_leader(level + 1, is_terminal),
                Style::BOLD,
                Style::MAGENTA,
                Style::RESET,
                Style::RESET_BOLD
            );

            self.print_statements(&stmt.else_stmt.as_ref().unwrap(), level + 2, false);
        }
    }

    fn print_expression(&self, expr: &ExpressionAnnotation, level: usize, is_terminal: bool) {
        match expr {
            ExpressionAnnotation::Binary(expr) => {
                self.print_binary_expression(expr, level, is_terminal);
            }

            ExpressionAnnotation::Unary(expr) => {
                self.print_unary_expression(expr, level, is_terminal);
            }

            ExpressionAnnotation::Atom(atom) => {
                self.print_atom(atom, level);
            }

            ExpressionAnnotation::Cast(cast) => {
                self.print_cast(cast, level, is_terminal);
            }

            ExpressionAnnotation::Null => {
                println!("NULL")
            }

            ExpressionAnnotation::Error => println!("Expression error"),
        }
    }

    fn print_binary_expression(&self, expr: &BinaryAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

        println!(
            "{leader}{}Expression{}({}{}{}) : {}{}{}{}{}",
            Style::BLUE,
            Style::RESET,
            Style::BRIGHT_CYAN,
            expr.op.operator,
            Style::RESET,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            expr.expr_type.display(),
            Style::RESET,
            Style::RESET_BOLD
        );

        self.print_expression(&expr.lhs, level + 1, false);
        self.print_expression(&expr.rhs, level + 1, is_terminal);
    }

    fn print_unary_expression(&self, expr: &UnaryAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, false);

        println!(
            "{leader}{}Expression{}({}{}{}) : {}{}{}{}{}",
            Style::BLUE,
            Style::RESET,
            Style::BRIGHT_CYAN,
            expr.op.operator,
            Style::RESET,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            expr.expr_type.display(),
            Style::RESET,
            Style::RESET_BOLD
        );

        self.print_expression(&expr.operand, level + 1, is_terminal);
    }

    fn print_cast(&self, cast: &CastAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, false);

        println!(
            "{leader}{}Cast{}[{}{}{}{}{} -> {}{}{}{}{}]",
            Style::BLUE,
            Style::RESET,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            cast.from.display(),
            Style::RESET,
            Style::RESET_BOLD,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            cast.to.display(),
            Style::RESET,
            Style::RESET_BOLD
        );

        self.print_expression(&cast.expr, level + 1, is_terminal);
    }

    fn print_atom(&self, expr: &AtomAnnotation, level: usize) {
        match expr {
            AtomAnnotation::Integer(atom) => self.print_integer(atom, level),

            AtomAnnotation::Float(atom) => self.print_float(atom, level),

            AtomAnnotation::Bool(atom) => self.print_bool(atom, level),

            AtomAnnotation::Null(_) => self.print_null(level),

            AtomAnnotation::Identifier(ident) => self.print_identifier(ident, level),
        }
    }

    fn print_integer(&self, integer: &IntegerAnnotation, level: usize) {
        let leader = Self::generate_field_leader(level, true);

        println!(
            "{leader}{}Atom{}({}{}{}) : {}{}{}{}{}",
            Style::CYAN,
            Style::RESET,
            Style::BRIGHT_YELLOW,
            integer.value,
            Style::RESET,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            integer.atom_type.display(),
            Style::RESET,
            Style::RESET_BOLD
        );
    }

    fn print_float(&self, float: &FloatAnnotation, level: usize) {
        let leader = Self::generate_field_leader(level, true);

        let value = if float.value.fract() == 0.0 {
            format!("{:.1}", float.value)
        } else {
            format!("{}", float.value)
        };

        println!(
            "{leader}{}Atom{}({}{}{}) : {}{}{}{}{}",
            Style::CYAN,
            Style::RESET,
            Style::BRIGHT_YELLOW,
            value,
            Style::RESET,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            float.atom_type.display(),
            Style::RESET,
            Style::RESET_BOLD
        );
    }

    fn print_bool(&self, bool: &BoolAnnotation, level: usize) {
        let leader = Self::generate_field_leader(level, true);

        println!(
            "{leader}{}Bool{}({}{}{}) : {}{}{}{}{}",
            Style::CYAN,
            Style::RESET,
            Style::BRIGHT_YELLOW,
            bool.value,
            Style::RESET,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            bool.atom_type.display(),
            Style::RESET,
            Style::RESET_BOLD
        );
    }

    fn print_identifier(&self, ident: &IdentifierAnnotation, level: usize) {
        let leader = Self::generate_field_leader(level, true);

        println!(
            "{leader}{}Atom{}({}{}{} {}[id: {}]{}) : {}{}{}{}{} ",
            Style::CYAN,
            Style::RESET,
            Style::BRIGHT_YELLOW,
            self.symbol_table.get_symbol(ident.span),
            Style::RESET,
            Style::DIM,
            ident.entry,
            Style::RESET_DIM,
            Style::BOLD,
            Style::BRIGHT_BLUE,
            ident.atom_type.display(),
            Style::RESET,
            Style::RESET_BOLD,
        );
    }

    fn print_null(&self, level: usize) {
        let leader = Self::generate_field_leader(level, true);

        println!("{leader}{}Null{}", Style::CYAN, Style::RESET,);
    }

    pub fn print_statements(&self, stmts: &AnnotatedStatements, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);
        println!(
            "{leader}{}{}Statements{}{}",
            Style::BOLD,
            Style::MAGENTA,
            Style::RESET,
            Style::RESET_BOLD
        );

        for statement in stmts.statements.iter() {
            self.print_statement(statement, level + 1, is_terminal);
        }
    }

    pub fn print(&self, tree: &AnnotatedTree) {
        println!(
            "{}{}Program{}{}",
            Style::BOLD,
            Style::MAGENTA,
            Style::RESET,
            Style::RESET_BOLD
        );

        for statement in tree.statements.statements.iter() {
            self.print_statement(statement, 1, false);
        }
    }
}
