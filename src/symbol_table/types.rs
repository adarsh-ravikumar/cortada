use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum BuiltinType {
    Integer,
    Float,
    Bool,
    Null,
}

impl BuiltinType {
    pub fn rank(&self) -> Option<u8> {
        match self {
            Self::Integer => Some(0),
            Self::Float => Some(1),
            _ => None,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            Self::Integer => "int",
            Self::Float => "float",
            Self::Bool => "bool",
            Self::Null => "null",
        }
    }

    pub fn try_implicit_cast(&self, target: &TypeKind) -> bool {
        match self {
            Self::Integer => match target {
                TypeKind::Builtin(ty) => matches!(ty, Self::Float),
                TypeKind::Union(union) => union.accepts(&TypeKind::Builtin(*self)),
                TypeKind::Error => true,
            },

            Self::Float => match target {
                TypeKind::Builtin(ty) => matches!(ty, Self::Float),
                TypeKind::Union(union) => union.accepts(&TypeKind::Builtin(*self)),
                TypeKind::Error => true,
            },

            Self::Bool => false,

            Self::Null => false,
        }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub struct UnionType {
    pub variants: BTreeSet<TypeKind>,
}

impl UnionType {
    pub fn accepts(&self, source: &TypeKind) -> bool {
        match source {
            TypeKind::Builtin(_) => self.variants.contains(source),
            TypeKind::Union(union) => self.variants.is_superset(&union.variants),
            TypeKind::Error => true,
        }
    }

    pub fn display(&self) -> String {
        self.variants
            .iter()
            .map(|variant| variant.display())
            .collect::<Vec<String>>()
            .join(" | ")
    }

    pub fn rank(&self) -> Option<u8> {
        let mut max: u8 = 0;

        for variant in &self.variants {
            match variant.rank() {
                None => return None,
                Some(rank) => {
                    if rank > max {
                        max = rank
                    }
                }
            }
        }

        Some(max)
    }
}

impl Clone for UnionType {
    fn clone(&self) -> Self {
        let mut variants: BTreeSet<TypeKind> = BTreeSet::new();

        for variant in self.variants.iter() {
            variants.insert(variant.clone());
        }

        UnionType { variants }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum TypeKind {
    Builtin(BuiltinType),
    Union(UnionType),
    Error,
}

impl TypeKind {
    pub fn accepts(&self, source: &TypeKind) -> bool {
        match (self, source) {
            (TypeKind::Builtin(lhs), TypeKind::Builtin(rhs)) => lhs == rhs,
            (TypeKind::Union(lhs), _) => lhs.accepts(source),
            _ => false,
        }
    }

    pub fn display(&self) -> String {
        match self {
            TypeKind::Builtin(ty) => ty.display().into(),
            TypeKind::Union(ty) => ty.display(),
            TypeKind::Error => "error".into(),
        }
    }

    pub fn try_implicit_cast(&self, target: &Self) -> bool {
        match self {
            TypeKind::Builtin(ty) => ty.try_implicit_cast(target),
            TypeKind::Union(ty) => ty.accepts(target),
            TypeKind::Error => true,
        }
    }

    pub fn rank(&self) -> Option<u8> {
        match self {
            TypeKind::Builtin(ty) => ty.rank(),
            TypeKind::Union(ty) => ty.rank(),
            TypeKind::Error => Some(0),
        }
    }
}
