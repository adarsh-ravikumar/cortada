use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone)]
pub enum BuiltinType {
    Integer,
    Float,
    Null,
}

impl BuiltinType {
    pub fn rank(&self) -> u8 {
        match self {
            Self::Null => 0,
            Self::Integer => 1,
            Self::Float => 2,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            Self::Integer => "int",
            Self::Float => "float",
            Self::Null => "null",
        }
    }

    pub fn try_cast(&self, target: &TypeKind) -> bool {
        match self {
            Self::Integer => match target {
                TypeKind::Builtin(ty) => matches!(ty, Self::Float),
                TypeKind::Union(_) => false,
            },

            Self::Float => match target {
                TypeKind::Builtin(ty) => matches!(ty, Self::Float),
                TypeKind::Union(_) => false,
            },

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
        }
    }

    pub fn display(&self) -> String {
        self.variants
            .iter()
            .map(|variant| variant.display())
            .collect::<Vec<String>>()
            .join(" | ")
    }

    pub fn rank(&self) -> u8 {
        let mut max: u8 = 0;

        for variant in &self.variants {
            let rank = variant.rank();
            if rank > max {
                max = rank
            }
        }

        max
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
        }
    }

    pub fn try_cast(&self, target: &Self) -> bool {
        match self {
            TypeKind::Builtin(ty) => ty.try_cast(target),
            TypeKind::Union(ty) => ty.accepts(target),
        }
    }

    pub fn rank(&self) -> u8 {
        match self {
            TypeKind::Builtin(ty) => ty.rank(),
            TypeKind::Union(ty) => ty.rank(),
        }
    }
}
