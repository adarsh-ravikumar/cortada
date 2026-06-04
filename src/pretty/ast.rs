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
                "{leader}{}Identifier{}({}{}{})",
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

            AstNodeKind::Statements(stmts) => {
                println!(
                    "{leader}{}{}Statements{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );
                for stmt in stmts.stmts.iter() {
                    Self::print_helper(&stmt, file, level + 1, is_terminal);
                }
            }

            AstNodeKind::If(stmt) => {
                println!(
                    "{leader}{}{}If{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                // condition
                println!(
                    "{}{}├── {}condition",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::MAGENTA,
                );

                let is_terminal = match stmt.condition.kind {
                    AstNodeKind::Integer(_)
                    | AstNodeKind::Float(_)
                    | AstNodeKind::Identifier(_) => true,
                    _ => false,
                };

                Self::print_helper(&stmt.condition, file, level + 2, is_terminal);

                // body
                Self::print_helper(&stmt.body, file, level + 1, is_terminal);

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
                        Self::generate_leader(level + 1),
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
                        Self::generate_leader(level + 2),
                        Style::MAGENTA,
                    );

                    let is_terminal = match stmt.condition.kind {
                        AstNodeKind::Integer(_)
                        | AstNodeKind::Float(_)
                        | AstNodeKind::Identifier(_) => true,
                        _ => false,
                    };

                    Self::print_helper(&elif_stmt.condition, file, level + 3, is_terminal);

                    // body
                    Self::print_helper(&elif_stmt.body, file, level + 2, is_terminal);
                }

                // else
                if stmt.else_stmt.is_some() {
                    println!(
                        "{}{}└── {}{}Else{}{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::BOLD,
                        Style::MAGENTA,
                        Style::RESET,
                        Style::RESET_BOLD
                    );

                    Self::print_helper(
                        stmt.else_stmt.as_ref().unwrap(),
                        file,
                        level + 2,
                        is_terminal,
                    );
                }
            }

            AstNodeKind::While(stmt) => {
                println!(
                    "{leader}{}{}While{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                // condition
                println!(
                    "{}{}├── {}condition",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::MAGENTA,
                );

                let is_terminal = match stmt.condition.kind {
                    AstNodeKind::Integer(_)
                    | AstNodeKind::Float(_)
                    | AstNodeKind::Identifier(_) => true,
                    _ => false,
                };

                Self::print_helper(&stmt.condition, file, level + 2, is_terminal);

                // body
                Self::print_helper(&stmt.body, file, level + 1, is_terminal);

                // else
                if stmt.else_stmt.is_some() {
                    println!(
                        "{}{}└── {}{}Else{}{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::BOLD,
                        Style::MAGENTA,
                        Style::RESET,
                        Style::RESET_BOLD
                    );

                    Self::print_helper(
                        stmt.else_stmt.as_ref().unwrap(),
                        file,
                        level + 2,
                        is_terminal,
                    );
                }
            }

            AstNodeKind::VarDecl(stmt) => {
                println!(
                    "{leader}{}{}VarDecl{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                println!(
                    "{}{}├── {}Name: {}{}{}",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::CYAN,
                    Style::BRIGHT_YELLOW,
                    file.view_span(stmt.name.span),
                    Style::RESET
                );

                if let Some(var_type) = &stmt.var_type {
                    println!(
                        "{}{}├── {}Type: {}{}{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::CYAN,
                        Style::BRIGHT_YELLOW,
                        file.view_span(var_type.span),
                        Style::RESET
                    );
                }

                print!(
                    "{}{}└── {}Value",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::CYAN
                );

                if let Some(v) = &stmt.value {
                    let is_terminal = match v.kind {
                        AstNodeKind::Integer(_)
                        | AstNodeKind::Float(_)
                        | AstNodeKind::Identifier(_) => true,
                        _ => false,
                    };

                    print!("{}\n", Style::RESET);
                    Self::print_helper(&v, file, level + 2, is_terminal);
                } else {
                    println!(": {}null{}", Style::BRIGHT_YELLOW, Style::RESET);
                }
            }

            AstNodeKind::VarAssign(stmt) => {
                println!(
                    "{leader}{}{}VarAssign{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                println!(
                    "{}{}├── {}Name: {}{}{}",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::CYAN,
                    Style::BRIGHT_YELLOW,
                    file.view_span(stmt.name.span),
                    Style::RESET
                );

                println!(
                    "{}{}└── {}Value {}",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::CYAN,
                    Style::RESET
                );

                let is_terminal = match stmt.value.kind {
                    AstNodeKind::Integer(_)
                    | AstNodeKind::Float(_)
                    | AstNodeKind::Identifier(_) => true,
                    _ => false,
                };

                Self::print_helper(&stmt.value, file, level + 2, is_terminal);
            }

            AstNodeKind::Fn(stmt) => {
                println!(
                    "{leader}{}{}Fn{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                println!(
                    "{}{}├── {}Name: {}{}{}",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::CYAN,
                    Style::BRIGHT_YELLOW,
                    file.view_span(stmt.name.span),
                    Style::RESET
                );

                if !stmt.params.is_empty() {
                    println!(
                        "{}{}├── {}Params{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::CYAN,
                        Style::RESET
                    );
                }

                for param in stmt.params.iter() {
                    println!(
                        "{}{}├── {}Name: {}{}{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 2),
                        Style::CYAN,
                        Style::BRIGHT_YELLOW,
                        file.view_span(param.name.span),
                        Style::RESET
                    );

                    if let Some(var_type) = &param.param_type {
                        println!(
                            "{}{}├── {}Type: {}{}{}",
                            Style::BRIGHT_BLACK,
                            Self::generate_leader(level + 3),
                            Style::CYAN,
                            Style::BRIGHT_YELLOW,
                            file.view_span(var_type.span),
                            Style::RESET
                        );
                    }

                    if let Some(v) = &param.default_value {
                        print!(
                            "{}{}└── {}Default Value",
                            Style::BRIGHT_BLACK,
                            Self::generate_leader(level + 3),
                            Style::CYAN
                        );

                        let is_terminal = match v.kind {
                            AstNodeKind::Integer(_)
                            | AstNodeKind::Float(_)
                            | AstNodeKind::Identifier(_) => true,
                            _ => false,
                        };

                        print!("{}\n", Style::RESET);
                        Self::print_helper(&v, file, level + 4, is_terminal);
                    }
                }

                println!(
                    "{}{}├── {}Body{}",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::MAGENTA,
                    Style::RESET
                );

                Self::print_helper(&stmt.body, file, level + 1, is_terminal);
            }
        }
    }

    pub fn print(ast: &Box<AstNode>, file: &IOFile) {
        Self::print_helper(ast, file, 0, false);
    }
}
