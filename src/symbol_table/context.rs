use core::fmt;
use std::collections::HashMap;

use crate::symbol_table::TypeKind;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ContextKind {
    Global,
    Function,
    Conditional,
    Loop,
}

pub struct Context<'a> {
    pub kind: ContextKind,
    pub symbols: HashMap<&'a str, Vec<usize>>,

    pub return_type: Option<TypeKind>,
    pub does_break: bool,
    pub does_continue: bool,

    pub parent: usize,
    pub children: Vec<usize>,
}

impl<'a> fmt::Debug for Context<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl<'a> Context<'a> {
    pub fn new(kind: ContextKind, parent: usize) -> Self {
        Self {
            kind,
            symbols: HashMap::new(),

            return_type: None,
            does_break: false,
            does_continue: false,

            parent,
            children: Vec::new(),
        }
    }
}

pub struct ContextStack<'a> {
    contexts: HashMap<usize, Context<'a>>,
    stack: Vec<usize>,
    next_id: usize,
}

impl<'a> ContextStack<'a> {
    pub fn new() -> Self {
        let mut new = Self {
            contexts: HashMap::new(),
            stack: vec![0],
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
        self.stack.push(id);

        self.next_id += 1;
    }

    pub fn exit_context(&mut self) {
        self.stack.pop();
    }

    pub fn get_current(&self) -> &Context<'a> {
        self.contexts.get(self.stack.last().unwrap()).unwrap()
    }

    pub fn get_mut_current(&mut self) -> &mut Context<'a> {
        self.contexts.get_mut(self.stack.last().unwrap()).unwrap()
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

    pub fn try_set_context_return(&mut self, return_type: TypeKind) -> bool {
        let mut current = self.get_current();

        loop {
            if current.kind == ContextKind::Global {
                return false;
            }

            if current.kind == ContextKind::Function {
                self.contexts
                    .get_mut(self.stack.last().unwrap())
                    .unwrap()
                    .return_type = Some(return_type);
                return true;
            }

            current = self.contexts.get(&current.parent).unwrap();
        }
    }

    pub fn try_set_context_break(&mut self) -> bool {
        let mut current = self.get_current();

        loop {
            if current.kind == ContextKind::Global {
                return false;
            }

            if current.kind == ContextKind::Loop {
                return true;
            }

            current = self.contexts.get(&current.parent).unwrap();
        }
    }

    pub fn try_set_context_continue(&mut self) -> bool {
        let mut current = self.get_current();

        loop {
            if current.kind == ContextKind::Global {
                return false;
            }

            if current.kind == ContextKind::Loop {
                return true;
            }

            current = self.contexts.get(&current.parent).unwrap();
        }
    }
}
