use polars::prelude::*;
use std::sync::LazyLock;

pub const FATTY_ACID: &str = "FattyAcid";

/// Fatty acids schema
pub static FATTY_ACIDS_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    Schema::from_iter([
        Field::new("Carbons".into(), DataType::UInt8),
        Field::new("Doubles".into(), DataType::List(Box::new(DataType::Int8))),
        Field::new("Triples".into(), DataType::List(Box::new(DataType::Int8))),
    ])
});

/// Extension methods for [`Schema`]
pub trait SchemaExt {
    fn names(&self) -> Vec<Expr>;
}

impl SchemaExt for Schema {
    fn names(&self) -> Vec<Expr> {
        self.iter_names_cloned().map(col).collect()
    }
}

pub mod column;
pub mod data_frame;
pub mod expr;
pub mod series;
