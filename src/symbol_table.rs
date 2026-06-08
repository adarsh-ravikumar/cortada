mod scope;
mod table;

mod binding;
mod types;

pub use binding::{BindingId, BindingTable};
pub use table::SymbolTable;
pub use types::{BuiltinType, TypeKind, UnionType};
