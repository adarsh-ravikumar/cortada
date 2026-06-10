use crate::semantic::SemanticAnalyzer;

use std::collections::BTreeSet;

use crate::{
    parser::{AstNodeKind, TypePrimary, TypePrimaryKind, TypeUnion},
    symbol_table::{BuiltinType, TypeKind, UnionType},
};

impl<'a> SemanticAnalyzer<'a> {
    pub fn annotate_type_expression(&mut self, ty: AstNodeKind) -> TypeKind {
        match ty {
            AstNodeKind::TypePrimary(primary) => self.annotate_type_primary(primary),
            AstNodeKind::TypeUnion(union) => self.annotate_type_union(union),
            _ => panic!("type of decleration must be a TypePrimary or TypeUnion node"),
        }
    }

    pub fn annotate_type_union(&mut self, union: TypeUnion) -> TypeKind {
        let mut variants: BTreeSet<TypeKind> = BTreeSet::new();

        for variant in union.variants {
            match variant.kind {
                AstNodeKind::TypePrimary(ty) => {
                    variants.insert(self.annotate_type_primary(ty));
                }

                AstNodeKind::TypeUnion(ty) => {
                    let union = self.annotate_type_union(ty);
                    match union {
                        TypeKind::Union(union) => variants.extend(union.variants),
                        _ => panic!(
                            "Annotate type union must return an annotation containing a union"
                        ),
                    };
                }

                _ => panic!("Union can only contain a type primary or a type union node"),
            }
        }

        TypeKind::Union(UnionType { variants })
    }

    pub fn annotate_type_primary(&mut self, ty: TypePrimary) -> TypeKind {
        match ty.kind {
            TypePrimaryKind::Integer => TypeKind::Builtin(BuiltinType::Integer),
            TypePrimaryKind::Float => TypeKind::Builtin(BuiltinType::Float),
            TypePrimaryKind::Bool => TypeKind::Builtin(BuiltinType::Bool),
        }
    }
}
