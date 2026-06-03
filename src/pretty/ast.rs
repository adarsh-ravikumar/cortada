use crate::{
    parser::{AstNode, AstNodeKind},
    utils::{IOFile, Style},
};

pub struct AstPrinter;

impl AstPrinter {
    fn generate_leader(level: usize) -> String {
        "│   ".repeat(level - 1)
    }

    fn print_helper(ast: &Box<AstNode>, file: &IOFile, level: usize, is_terminal: bool) {
        let leader = if level > 0 {
            format!(
                "{}{}{} {}",
                Style::BRIGHT_BLACK,
                Self::generate_leader(level),
                if is_terminal {
                    "└──"
                } else {
                    "├──"
                },
                Style::RESET
            )
        } else {
            "".to_string()
        };

        match &ast.kind {
            AstNodeKind::Integer(expr) => println!(
                "{leader}{}Integer{}({}{}{})",
                Style::CYAN,
                Style::RESET,
                Style::BRIGHT_YELLOW,
                expr.value,
                Style::RESET
            ),

            AstNodeKind::Float(expr) => println!(
                "{leader}{}Float{}({}{}{})",
                Style::CYAN,
                Style::RESET,
                Style::BRIGHT_YELLOW,
                expr.value,
                Style::RESET
            ),

            AstNodeKind::Identifier(expr) => println!(
                "{leader}{}Float{}({}{}{})",
                Style::CYAN,
                Style::RESET,
                Style::BRIGHT_YELLOW,
                file.view_span(expr.span),
                Style::RESET
            ),

            AstNodeKind::Binary(expr) => {
                println!(
                    "{leader}{}{}Binary{}{}",
                    Style::BOLD,
                    Style::BLUE,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                Self::print_helper(&expr.lhs, file, level + 1, false);

                println!(
                    "{}{}├── {}op{}({}{}{})",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::MAGENTA,
                    Style::RESET,
                    Style::BRIGHT_CYAN,
                    expr.op,
                    Style::RESET,
                );

                let is_terminal = match expr.rhs.kind {
                    AstNodeKind::Integer(_)
                    | AstNodeKind::Float(_)
                    | AstNodeKind::Identifier(_) => true,
                    _ => false,
                };

                Self::print_helper(&expr.rhs, file, level + 1, is_terminal);
            }

            AstNodeKind::Unary(expr) => {
                println!(
                    "{leader}{}{}Unary{}{}",
                    Style::BOLD,
                    Style::GREEN,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                println!(
                    "{}{}├── {}op{}({}{}{})",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::MAGENTA,
                    Style::RESET,
                    Style::BRIGHT_CYAN,
                    expr.op,
                    Style::RESET,
                );

                let is_terminal = match expr.rhs.kind {
                    AstNodeKind::Integer(_)
                    | AstNodeKind::Float(_)
                    | AstNodeKind::Identifier(_) => true,
                    _ => false,
                };

                Self::print_helper(&expr.rhs, file, level + 1, is_terminal);
            }
        }
    }

    pub fn print(ast: &Box<AstNode>, file: &IOFile) {
        Self::print_helper(ast, file, 0, false);
    }
}
