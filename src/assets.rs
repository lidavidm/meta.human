use std;
use yaml_rust;

#[derive(Debug)]
pub enum DecodeError {
    TypeError(String),
}

pub type Result<T> = std::result::Result<T, DecodeError>;

pub trait Decodable: Sized {
    fn decode(doc: &yaml_rust::Yaml) -> Result<Self>;
}

macro_rules! get_field {
    ( $doc: expr, $field: expr, $as_: ident ) => {
        as_value!( $doc[$field], $as_ )
    }
}

macro_rules! as_value {
    ( $doc: expr, $as_: ident ) => {
        try!($doc.$as_().ok_or(assets::DecodeError::TypeError(format!("Wrong value type"))))
    }
}
