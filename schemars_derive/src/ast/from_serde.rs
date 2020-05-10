use super::*;
use crate::attr::get_with_from_attrs;
use serde_derive_internals::ast as serde_ast;
use serde_derive_internals::Ctxt;

pub trait FromSerde: Sized {
    type SerdeType;

    fn from_serde(errors: &Ctxt, serde: Self::SerdeType) -> Result<Self, ()>;

    fn vec_from_serde(errors: &Ctxt, serdes: Vec<Self::SerdeType>) -> Result<Vec<Self>, ()> {
        let mut result = Vec::with_capacity(serdes.len());
        for s in serdes {
            result.push(Self::from_serde(errors, s)?)
        }
        Ok(result)
    }
}

impl<'a> FromSerde for Container<'a> {
    type SerdeType = serde_ast::Container<'a>;

    fn from_serde(errors: &Ctxt, serde: Self::SerdeType) -> Result<Self, ()> {
        Ok(Self {
            ident: serde.ident,
            serde_attrs: serde.attrs,
            data: Data::from_serde(errors, serde.data)?,
            generics: serde.generics,
            original: serde.original,
        })
    }
}

impl<'a> FromSerde for Data<'a> {
    type SerdeType = serde_ast::Data<'a>;

    fn from_serde(errors: &Ctxt, serde: Self::SerdeType) -> Result<Self, ()> {
        Ok(match serde {
            Self::SerdeType::Enum(variants) => {
                Self::Enum(Variant::vec_from_serde(errors, variants)?)
            }
            Self::SerdeType::Struct(style, fields) => {
                Self::Struct(style, Field::vec_from_serde(errors, fields)?)
            }
        })
    }
}

impl<'a> FromSerde for Variant<'a> {
    type SerdeType = serde_ast::Variant<'a>;

    fn from_serde(errors: &Ctxt, serde: Self::SerdeType) -> Result<Self, ()> {
        Ok(Self {
            ident: serde.ident,
            serde_attrs: serde.attrs,
            style: serde.style,
            fields: Field::vec_from_serde(errors, serde.fields)?,
            original: serde.original,
            with: get_with_from_attrs(&serde.original.attrs, errors)?,
        })
    }
}

impl<'a> FromSerde for Field<'a> {
    type SerdeType = serde_ast::Field<'a>;

    fn from_serde(errors: &Ctxt, serde: Self::SerdeType) -> Result<Self, ()> {
        Ok(Self {
            member: serde.member,
            serde_attrs: serde.attrs,
            ty: serde.ty,
            original: serde.original,
            with: get_with_from_attrs(&serde.original.attrs, errors)?,
        })
    }
}