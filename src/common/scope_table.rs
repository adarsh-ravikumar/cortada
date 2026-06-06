use std::collections::HashMap;

pub struct ScopeTable<'a> {
    table: HashMap<usize, HashMap<&'a str, usize>>,
    next_scope_id: usize,
}

impl<'a> ScopeTable<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            next_scope_id: 0,
        }
    }

    pub fn create_scope(&mut self) -> usize {
        let id = self.next_scope_id;

        self.table.insert(id, HashMap::new());
        self.next_scope_id += 1;

        id
    }

    pub fn symbol_id(&self, scope_id: usize, symbol: &'a str) -> Option<&usize> {
        // right now, we do scope checking naively
        // later, we will implement logic for upward propogation
        let scope = match self.table.get(&scope_id) {
            Some(scope) => scope,
            None => return None,
        };

        scope.get(&symbol)
    }
}
