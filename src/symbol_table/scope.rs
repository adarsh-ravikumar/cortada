use std::collections::HashMap;

pub struct ScopeTable<'a> {
    table: HashMap<&'a str, usize>,
}

impl<'a> ScopeTable<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: &'a str, id: usize) {
        self.table.insert(symbol, id);
    }

    pub fn get_id(&self, symbol: &'a str) -> Option<&usize> {
        self.table.get(symbol)
    }
}
