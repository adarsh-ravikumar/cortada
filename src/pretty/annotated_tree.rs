use crate::{
    parser::AstNodeKind,
    semantic::{
        AnnotatedTree, AtomAnnotation, BinaryAnnotation, CastAnnotation, ExpressionAnnotation,
        FloatAnnotation, IntegerAnnotation, StatementAnnotation, UnaryAnnotation,
    },
    utils::Style,
};

pub struct AnnotatedTreePrinter;

impl AnnotatedTreePrinter {
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
        matches!(
            kind,
            AstNodeKind::Integer(_) | AstNodeKind::Float(_) | AstNodeKind::Identifier
        )
    }

    fn print_statement(stmt: &StatementAnnotation, level: usize, is_terminal: bool) {
        match stmt {
            StatementAnnotation::Expression(expr) => {
                Self::print_expression(expr, level, is_terminal);
            }

            _ => panic!("not implemented"),
        }
    }

    fn print_expression(expr: &ExpressionAnnotation, level: usize, is_terminal: bool) {
        match expr {
            ExpressionAnnotation::Binary(expr) => {
                Self::print_binary_expression(expr, level, is_terminal);
            }

            ExpressionAnnotation::Unary(expr) => {
                Self::print_unary_expression(expr, level, is_terminal);
            }

            ExpressionAnnotation::Atom(atom) => {
                Self::print_atom(atom, level, is_terminal);
            }

            ExpressionAnnotation::Cast(cast) => {
                Self::print_cast(cast, level, is_terminal);
            }
        }
    }

    fn print_binary_expression(expr: &BinaryAnnotation, level: usize, is_terminal: bool) {
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

        Self::print_expression(&expr.lhs, level + 1, false);
        Self::print_expression(&expr.rhs, level + 1, is_terminal);
    }

    fn print_unary_expression(expr: &UnaryAnnotation, level: usize, is_terminal: bool) {
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

        Self::print_expression(&expr.operand, level + 1, is_terminal);
    }

    fn print_cast(cast: &CastAnnotation, level: usize, is_terminal: bool) {
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

        Self::print_expression(&cast.expr, level + 1, is_terminal);
    }

    fn print_atom(expr: &AtomAnnotation, level: usize, is_terminal: bool) {
        match expr {
            AtomAnnotation::Integer(atom) => Self::print_integer(atom, level, is_terminal),
            AtomAnnotation::Float(atom) => Self::print_float(atom, level, is_terminal),
            _ => panic!("unimplemented"),
        }
    }

    fn print_integer(integer: &IntegerAnnotation, level: usize, is_terminal: bool) {
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

    fn print_float(float: &FloatAnnotation, level: usize, is_terminal: bool) {
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

    pub fn print(tree: &AnnotatedTree) {
        println!(
            "{}{}Statements{}{}",
            Style::BOLD,
            Style::MAGENTA,
            Style::RESET,
            Style::RESET_BOLD
        );

        for statement in tree.statements.iter() {
            Self::print_statement(statement, 1, false);
        }
    }
}
