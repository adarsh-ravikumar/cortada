#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Type {
    Integer,
    Float,
    Null,
}

impl Type {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Integer => "int",
            Self::Float => "float",
            Self::Null => "null",
        }
    }

    pub fn try_cast(&self, target: Self) -> bool {
        match self {
            Self::Integer => {
                matches!(target, Self::Float)
            }

            Self::Float => {
                matches!(target, Self::Integer)
            }

            Self::Null => false,
        }
    }
}
