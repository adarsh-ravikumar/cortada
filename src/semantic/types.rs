use crate::semantic::SemanticAnalyzer;

use std::collections::BTreeSet;

use crate::{
    diagnostic::Diagnostic,
    parser::{AstNodeKind, TypePrimary, TypePrimaryKind, TypeUnion},
    symbol_table::{BuiltinType, TypeKind, UnionType},
};

impl<'a> SemanticAnalyzer<'a> {
    pub fn annotate_type_expression(&mut self, ty: AstNodeKind) -> Result<TypeKind, Diagnostic> {
        match ty {
            AstNodeKind::TypePrimary(primary) => Ok(self.annotate_type_primary(primary)?),
            AstNodeKind::TypeUnion(union) => Ok(self.annotate_type_union(union)?),
            _ => panic!("type of decleration must be a TypePrimary or TypeUnion node"),
        }
    }

    pub fn annotate_type_union(&mut self, union: TypeUnion) -> Result<TypeKind, Diagnostic> {
        let mut variants: BTreeSet<TypeKind> = BTreeSet::new();

        for variant in union.variants {
            match variant.kind {
                AstNodeKind::TypePrimary(ty) => {
                    variants.insert(self.annotate_type_primary(ty)?);
                }

                AstNodeKind::TypeUnion(ty) => {
                    let union = self.annotate_type_union(ty)?;
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

        Ok(TypeKind::Union(UnionType { variants }))
    }

    pub fn annotate_type_primary(&mut self, ty: TypePrimary) -> Result<TypeKind, Diagnostic> {
        match ty.kind {
            TypePrimaryKind::Integer => Ok(TypeKind::Builtin(BuiltinType::Integer)),
            TypePrimaryKind::Float => Ok(TypeKind::Builtin(BuiltinType::Float)),
            TypePrimaryKind::Bool => Ok(TypeKind::Builtin(BuiltinType::Bool)),
        }
    }
}
