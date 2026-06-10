mod scope;
mod table;

mod binding;
mod function;
mod types;

pub use binding::{BindingEntry, BindingId, BindingTable};
pub use function::{FunctionEntry, FunctionId, FunctionTable};
pub use scope::{ScopeEntryKind, ScopeTable};
pub use table::SymbolTable;
pub use types::{BuiltinType, TypeKind, UnionType};
