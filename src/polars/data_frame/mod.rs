use crate::polars::{FATTY_ACID, column::ColumnExt as _, series::fatty_acids::FattyAcidSeries};
use polars::prelude::*;

/// Extension methods for [`DataFrame`]
pub trait DataFrameExt {
    fn fatty_acid(&self) -> FattyAcidSeries;
}

impl DataFrameExt for DataFrame {
    fn fatty_acid(&self) -> FattyAcidSeries {
        self[FATTY_ACID].fatty_acid()
    }
}
