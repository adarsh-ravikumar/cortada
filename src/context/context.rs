use std::collections::HashMap;

use crate::{
    common::Span,
    context::{BuiltinType, TypeKind},
    diagnostic::{Diagnostic, DiagnosticClass, DiagnosticSeverity, Label, LabelKind},
};

#[derive(PartialEq, Eq, Clone)]
pub struct FunctionContext {
    pub return_type: TypeKind,
    pub annotated_return: Option<Span>,
    pub inferred_return: Option<Span>,
}

impl FunctionContext {
    pub fn get_return_span(&self) -> Option<Span> {
        if self.annotated_return.is_some() {
            self.annotated_return
        } else {
            self.inferred_return
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct LoopContext {
    pub does_break: bool,
    pub does_continue: bool,
}

#[derive(PartialEq, Eq, Clone)]
pub enum ContextKind {
    Global,
    Function(FunctionContext),
    Conditional,
    Loop(LoopContext),
}

impl ContextKind {
    pub fn function_context() -> Self {
        Self::Function(FunctionContext {
            return_type: TypeKind::Builtin(BuiltinType::Null),
            annotated_return: None,
            inferred_return: None,
        })
    }

    pub fn loop_context() -> Self {
        Self::Loop(LoopContext {
            does_break: false,
            does_continue: false,
        })
    }
}

pub struct Context<'a> {
    pub kind: ContextKind,
    pub symbols: HashMap<&'a str, Vec<usize>>,

    pub parent: usize,
    pub children: Vec<usize>,
}

impl<'a> Context<'a> {
    pub fn new(kind: ContextKind, parent: usize) -> Self {
        Self {
            kind,
            symbols: HashMap::new(),
            parent,
            children: Vec::new(),
        }
    }
}

pub struct ContextStack<'a> {
    contexts: HashMap<usize, Context<'a>>,
    stack: Vec<usize>,
    next_id: usize,
    pub current: usize,
}

impl<'a> ContextStack<'a> {
    pub fn new() -> Self {
        let mut new = Self {
            contexts: HashMap::new(),
            stack: vec![0],
            current: 0,
            next_id: 1,
        };

        new.contexts.insert(0, Context::new(ContextKind::Global, 0));

        new
    }

    pub fn enter_context(&mut self, kind: ContextKind) {
        let current_id = self.stack.last().unwrap();

        let id = self.next_id;

        self.contexts.insert(id, Context::new(kind, *current_id));

        let current_context = self.contexts.get_mut(current_id).unwrap();

        current_context.children.push(id);

        self.current = id;
        self.stack.push(id);

        self.next_id += 1;
    }

    pub fn exit_context(&mut self) {
        self.current = self.stack.pop().unwrap();
    }

    pub fn get_current(&self) -> &Context<'a> {
        self.contexts.get(self.stack.last().unwrap()).unwrap()
    }

    pub fn get_mut_current(&mut self) -> &mut Context<'a> {
        self.contexts.get_mut(self.stack.last().unwrap()).unwrap()
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Context<'a> {
        self.contexts.get_mut(&id).unwrap()
    }

    // returns 0 if failure
    pub fn resolve_in_parent(&self, symbol: &'a str) -> usize {
        for context in self.stack.iter().rev() {
            let context = self.contexts.get(context).unwrap();
            let symbols = &context.symbols;

            if let Some(scope_entry) = symbols.get(symbol) {
                return *scope_entry.last().unwrap();
            }
        }

        0
    }

    // returns 0 if failure
    pub fn resolve_in_child(&self, symbol: &'a str) -> usize {
        let current = self.get_current();

        for context in &current.children {
            let context = self.contexts.get(context).unwrap();
            let symbols = &context.symbols;

            if let Some(scope_entry) = symbols.get(symbol) {
                return *scope_entry.last().unwrap();
            }
        }

        0
    }

    pub fn try_set_context_return(
        &mut self,
        return_type: TypeKind,
        return_span: Span,
        expr_span: Span,
        is_inferred: bool,
    ) -> Option<Diagnostic> {
        let mut current = self.current;

        loop {
            let parent;

            {
                let current = self.get_mut(current);
                if current.kind == ContextKind::Global {
                    break;
                }

                if let ContextKind::Function(ctx) = &mut current.kind {
                    let return_type_known =
                        ctx.annotated_return.is_some() || ctx.inferred_return.is_some();

                    if return_type_known {
                        if ctx.return_type.accepts(&return_type) {
                            return None;
                        }

                        return Some(Diagnostic {
                            severity: DiagnosticSeverity::Error,
                            class: DiagnosticClass::TypeMismatch,
                            location: expr_span,
                            msg: format!(
                                "cannot return a value of type `{}` from a function returning `{}`",
                                return_type.display(),
                                ctx.return_type.display(),
                            ),

                            labels: vec![
                                Label {
                                    span: ctx.get_return_span().unwrap_or(Span::new(0, 0)),
                                    msg: format!(
                                        "function return type established as `{}` here",
                                        ctx.return_type.display(),
                                    ),
                                    kind: LabelKind::Secondary,
                                    paranthesise: false,
                                },

                                Label {
                                    span: expr_span,
                                    msg: format!(
                                        "this expression has type `{}`",
                                        return_type.display(),
                                    ),

                                    kind: LabelKind::Primary,
                                    paranthesise: true,
                                },
                            ],

                            notes: vec![
                                "all return statements must be compatible with the function's return type".into(),
                                "change the expression to produce the expected type, or adjust the function's return type".into(),
                            ],
                        });
                    }

                    // return type is unknown
                    if is_inferred {
                        ctx.inferred_return = Some(expr_span);
                    } else {
                        ctx.annotated_return = Some(return_span);
                    }

                    ctx.return_type = return_type.clone();
                }

                parent = current.parent;
            }

            current = parent;
        }

        Some(Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::InvalidControlFlow,
            location: return_span,
            msg: "`return` can only be used inside a function".into(),

            labels: vec![Label {
                span: return_span,
                msg: "not inside a function".into(),
                kind: LabelKind::Primary,
                paranthesise: false,
            }],

            notes: vec![
                "`return` transfers control to the caller of the enclosing function".into(),
                "remove the `return` statement, or move it into a function body".into(),
            ],
        })
    }

    pub fn try_set_context_break(&mut self, span: Span) -> Option<Diagnostic> {
        let mut current = *self.stack.last().unwrap();

        loop {
            let parent;

            {
                let current = self.get_mut(current);
                if current.kind == ContextKind::Global {
                    break;
                }
                if let ContextKind::Loop(ctx) = &mut current.kind {
                    ctx.does_break = true;
                    return None;
                }

                parent = current.parent;
            }

            current = parent;
        }

        Some(Diagnostic {
            severity: DiagnosticSeverity::Error,
            class: DiagnosticClass::InvalidControlFlow,
            location: span,
            msg: "`break` can only be used inside a loop".into(),

            labels: vec![Label {
                span: span,
                msg: "not inside a loop".into(),
                kind: LabelKind::Primary,
                paranthesise: false,
            }],
            notes: vec![
                "`break` terminates execution of the enclosing loop".into(),
                "remove the `break` statement, or place it inside a loop".into(),
            ],
        })
    }

    pub fn try_set_context_continue(&mut self, span: Span) -> Option<Diagnostic> {
        let mut current = *self.stack.last().unwrap();

        loop {
            let parent;

            {
                let current = self.get_mut(current);
                if current.kind == ContextKind::Global {
                    break;
                }
                if let ContextKind::Loop(ctx) = &mut current.kind {
                    ctx.does_continue = true;
                    return None;
                }

                parent = current.parent;
            }

            current = parent;
        }

        Some(Diagnostic {
                severity: DiagnosticSeverity::Error,
                class: DiagnosticClass::InvalidControlFlow,
                location: span,
                msg: "`continue` can only be used inside a loop".into(),

                labels: vec![Label {
                    span: span,
                    msg: "not inside a loop".into(),
                    kind: LabelKind::Primary,
                    paranthesise: false,
                }],

                notes: vec![
                    "`continue` skips the remainder of the current iteration and proceeds to the next one".into(),
                    "remove the `continue` statement, or place it inside a loop".into(),
                ],
            })
    }
}
