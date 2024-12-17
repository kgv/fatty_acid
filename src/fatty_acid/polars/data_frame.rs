use super::{COLUMN, column::ColumnExt as _, series::FattyAcidSeries};
use polars::prelude::*;

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid(&self) -> FattyAcidSeries;
}

impl DataFrameExt for DataFrame {
    fn fatty_acid(&self) -> FattyAcidSeries {
        self[COLUMN].fatty_acid()
    }
}
