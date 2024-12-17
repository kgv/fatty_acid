use crate::polars::series::{SeriesExt as _, fatty_acids::FattyAcidSeries};
use polars::prelude::*;

/// Extension methods for [`Column`]
pub trait ColumnExt {
    fn fatty_acid(&self) -> FattyAcidSeries;
}

impl ColumnExt for Column {
    fn fatty_acid(&self) -> FattyAcidSeries {
        self.as_materialized_series().fatty_acid()
    }
}