use std::collections::HashMap;

pub struct ScopeTable<'src, 'scope> {
    table: HashMap<&'src str, Vec<usize>>,
    parent: Option<&'scope Box<Self>>,
    can_propogate: bool,
}

impl<'src, 'scope> ScopeTable<'src, 'scope> {
    pub fn new(
        parent: Option<&'scope Box<ScopeTable<'src, 'scope>>>,
        can_propogate: bool,
    ) -> Box<Self> {
        Box::new(Self {
            table: HashMap::new(),
            parent,
            can_propogate,
        })
    }

    pub fn add_symbol(&mut self, symbol: &'src str, id: usize) {
        if let Some(existing) = self.table.get_mut(symbol) {
            existing.push(id)
        }

        self.table.insert(symbol, vec![id]);
    }

    pub fn get_id(&self, symbol: &'src str) -> Option<&usize> {
        if let Some(id) = self.table.get(symbol) {
            return Some(id.last().unwrap());
        }

        if self.can_propogate {
            self.parent.as_ref()?.get_id(symbol)
        } else {
            None
        }
    }
}
