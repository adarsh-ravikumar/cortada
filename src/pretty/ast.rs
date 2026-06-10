use crate::{
    common::IOFile,
    parser::{AstNode, AstNodeKind, AtomKind, TypePrimaryKind},
    utils::Style,
};

pub struct AstPrinter;

impl AstPrinter {
    fn generate_leader(level: usize) -> String {
        "│   ".repeat(level - 1)
    }

    fn is_terminal(kind: &AstNodeKind) -> bool {
        matches!(kind, AstNodeKind::Atom(_))
    }

    fn print_helper(ast: &Box<AstNode>, file: &IOFile, level: usize) {
        let leader = if level > 0 {
            format!(
                "{}{}{} {}",
                Style::BRIGHT_BLACK,
                Self::generate_leader(level),
                if Self::is_terminal(&ast.kind) {
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
            AstNodeKind::Error => (),
            AstNodeKind::Atom(atom) => match atom {
                AtomKind::Integer(val) => println!(
                    "{leader}{}Integer{}({}{}{})",
                    Style::CYAN,
                    Style::RESET,
                    Style::BRIGHT_YELLOW,
                    val,
                    Style::RESET
                ),

                AtomKind::Float(val) => println!(
                    "{leader}{}Float{}({}{}{})",
                    Style::CYAN,
                    Style::RESET,
                    Style::BRIGHT_YELLOW,
                    val,
                    Style::RESET
                ),

                AtomKind::Bool(val) => println!(
                    "{leader}{}Bool{}({}{}{})",
                    Style::CYAN,
                    Style::RESET,
                    Style::BRIGHT_YELLOW,
                    val,
                    Style::RESET
                ),

                AtomKind::Identifier => println!(
                    "{leader}{}Identifier{}({}{}{})",
                    Style::CYAN,
                    Style::RESET,
                    Style::BRIGHT_YELLOW,
                    file.view_span(ast.span),
                    Style::RESET
                ),

                AtomKind::Null => println!("{leader}{}Null{}", Style::CYAN, Style::RESET,),
            },

            AstNodeKind::TypeUnion(union) => {
                if union.variants.len() == 1 {
                    Self::print_helper(&union.variants.last().unwrap(), file, level);
                }

                println!("{leader}{}TypeUnion{}", Style::CYAN, Style::RESET);

                for variant in union.variants.iter() {
                    Self::print_helper(variant, file, level + 1);
                }
            }

            AstNodeKind::TypePrimary(expr) => match expr.kind {
                TypePrimaryKind::Integer => {
                    println!("{leader}{}Integer{}", Style::CYAN, Style::RESET)
                }

                TypePrimaryKind::Float => {
                    println!("{leader}{}Float{}", Style::CYAN, Style::RESET)
                }

                TypePrimaryKind::Bool => {
                    println!("{leader}{}Bool{}", Style::CYAN, Style::RESET)
                }
            },

            AstNodeKind::Binary(expr) => {
                println!(
                    "{leader}{}{}Binary{}({}{}{}{}{}){}",
                    Style::BOLD,
                    Style::BLUE,
                    Style::RESET,
                    Style::RESET_BOLD,
                    Style::BRIGHT_CYAN,
                    expr.op,
                    Style::RESET,
                    Style::BOLD,
                    Style::RESET_BOLD,
                );

                Self::print_helper(&expr.lhs, file, level + 1);

                Self::print_helper(&expr.rhs, file, level + 1);
            }

            AstNodeKind::Unary(expr) => {
                println!(
                    "{leader}{}{}Unary{}({}{}{}{}{}){}",
                    Style::BOLD,
                    Style::BLUE,
                    Style::RESET,
                    Style::RESET_BOLD,
                    Style::BRIGHT_CYAN,
                    expr.op,
                    Style::RESET,
                    Style::BOLD,
                    Style::RESET_BOLD,
                );

                Self::print_helper(&expr.operand, file, level + 1);
            }

            AstNodeKind::Program(program) => {
                println!(
                    "{leader}{}{}Program{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                Self::print_helper(&program.statements, file, level + 1);
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
                    Self::print_helper(&stmt, file, level + 1);
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

                Self::print_helper(&stmt.condition, file, level + 2);

                // body
                Self::print_helper(&stmt.body, file, level + 1);

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

                    Self::print_helper(&elif_stmt.condition, file, level + 3);

                    // body
                    Self::print_helper(&elif_stmt.body, file, level + 2);
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

                    Self::print_helper(stmt.else_stmt.as_ref().unwrap(), file, level + 2);
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

                Self::print_helper(&stmt.condition, file, level + 2);

                // body
                Self::print_helper(&stmt.body, file, level + 1);

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

                    Self::print_helper(stmt.else_stmt.as_ref().unwrap(), file, level + 2);
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
                    file.view_span(stmt.name),
                    Style::RESET
                );

                if let Some(var_type) = &stmt.var_type {
                    println!(
                        "{}{}├── {}Type{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::CYAN,
                        Style::RESET
                    );

                    Self::print_helper(&var_type, file, level + 2);
                }

                if let Some(value) = &stmt.value {
                    print!(
                        "{}{}├── {}Value",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::CYAN
                    );

                    print!("{}\n", Style::RESET);
                    Self::print_helper(value, file, level + 2);
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
                    file.view_span(stmt.name),
                    Style::RESET
                );

                println!(
                    "{}{}├── {}Value {}",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::CYAN,
                    Style::RESET
                );

                Self::print_helper(&stmt.value, file, level + 2);
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
                    file.view_span(stmt.name),
                    Style::RESET
                );

                if let Some(t) = &stmt.return_type {
                    println!(
                        "{}{}├── {}Return Type{}{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::CYAN,
                        Style::BRIGHT_YELLOW,
                        Style::RESET
                    );

                    Self::print_helper(t, file, level + 2);
                }

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
                        file.view_span(param.name),
                        Style::RESET
                    );

                    if let Some(param_type) = &param.param_type {
                        println!(
                            "{}{}├── {}Type{}",
                            Style::BRIGHT_BLACK,
                            Self::generate_leader(level + 1),
                            Style::CYAN,
                            Style::RESET
                        );

                        Self::print_helper(&param_type, file, level + 2);
                    }

                    if let Some(v) = &param.default_value {
                        print!(
                            "{}{}└── {}Default Value",
                            Style::BRIGHT_BLACK,
                            Self::generate_leader(level + 3),
                            Style::CYAN
                        );

                        print!("{}\n", Style::RESET);
                        Self::print_helper(&v, file, level + 4);
                    }
                }

                println!(
                    "{}{}├── {}Body{}",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::MAGENTA,
                    Style::RESET
                );

                Self::print_helper(&stmt.body, file, level + 2);
            }

            AstNodeKind::Call(stmt) => {
                println!(
                    "{leader}{}{}Call{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                println!(
                    "{}{}├── {}Callee",
                    Style::BRIGHT_BLACK,
                    Self::generate_leader(level + 1),
                    Style::CYAN,
                );

                Self::print_helper(&stmt.callee, file, level + 2);

                if !stmt.args.is_empty() {
                    println!(
                        "{}{}├── {}Arguments{}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 1),
                        Style::CYAN,
                        Style::RESET
                    );
                }

                for (idx, arg) in stmt.args.iter().enumerate() {
                    println!(
                        "{}{}├── {}Argument {}({}){}",
                        Style::BRIGHT_BLACK,
                        Self::generate_leader(level + 2),
                        Style::CYAN,
                        Style::BRIGHT_BLACK,
                        idx,
                        Style::RESET,
                    );

                    Self::print_helper(&arg, file, level + 4);
                }
            }

            AstNodeKind::Return(stmt) => {
                println!(
                    "{leader}{}{}Return{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );

                if let Some(e) = &stmt.expr {
                    Self::print_helper(e, file, level + 1);
                }
            }

            AstNodeKind::Break => {
                println!(
                    "{leader}{}{}Break{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );
            }

            AstNodeKind::Continue => {
                println!(
                    "{leader}{}{}Continue{}{}",
                    Style::BOLD,
                    Style::MAGENTA,
                    Style::RESET,
                    Style::RESET_BOLD
                );
            }
        }
    }

    pub fn print(ast: &Box<AstNode>, file: &IOFile) {
        Self::print_helper(ast, file, 0);
    }
}
