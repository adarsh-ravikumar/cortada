use crate::{
    parser::AstNodeKind,
    semantic::{
        AnnotatedStatements, AnnotatedTree, AtomAnnotation, BinaryAnnotation, BoolAnnotation,
        CastAnnotation, ExpressionAnnotation, FloatAnnotation, IdentifierAnnotation,
        IntegerAnnotation, StatementAnnotation, UnaryAnnotation, VarAssignAnnotation,
        VarDeclAnnotation,
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

    fn print_expression(&self, expr: &ExpressionAnnotation, level: usize, is_terminal: bool) {
        match expr {
            ExpressionAnnotation::Binary(expr) => {
                self.print_binary_expression(expr, level, is_terminal);
            }

            ExpressionAnnotation::Unary(expr) => {
                self.print_unary_expression(expr, level, is_terminal);
            }

            ExpressionAnnotation::Atom(atom) => {
                self.print_atom(atom, level, is_terminal);
            }

            ExpressionAnnotation::Cast(cast) => {
                self.print_cast(cast, level, is_terminal);
            }
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

    fn print_atom(&self, expr: &AtomAnnotation, level: usize, is_terminal: bool) {
        match expr {
            AtomAnnotation::Integer(atom) => self.print_integer(atom, level, is_terminal),

            AtomAnnotation::Float(atom) => self.print_float(atom, level, is_terminal),

            AtomAnnotation::Bool(atom) => self.print_bool(atom, level, is_terminal),

            AtomAnnotation::Null(_) => self.print_null(level, is_terminal),

            AtomAnnotation::Identifier(ident) => self.print_identifier(ident, level, is_terminal),
        }
    }

    fn print_integer(&self, integer: &IntegerAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

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

    fn print_float(&self, float: &FloatAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

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

    fn print_bool(&self, bool: &BoolAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

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

    fn print_identifier(&self, ident: &IdentifierAnnotation, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

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

    fn print_null(&self, level: usize, is_terminal: bool) {
        let leader = Self::generate_field_leader(level, is_terminal);

        println!("{leader}{}Null{}", Style::CYAN, Style::RESET,);
    }

    pub fn print_statements(&self, stmts: &AnnotatedStatements) {
        println!(
            "{}{}Statements{}{}",
            Style::BOLD,
            Style::MAGENTA,
            Style::RESET,
            Style::RESET_BOLD
        );

        for statement in stmts.statements.iter() {
            self.print_statement(statement, 1, false);
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
