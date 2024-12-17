use crate::fatty_acid::Unsaturated;
use polars::prelude::*;

/// Unsaturated series
#[derive(Clone, Debug)]
pub struct UnsaturatedSeries {
    pub index: Series,
    pub isomerism: Series,
    pub unsaturation: Series,
}

impl UnsaturatedSeries {
    pub fn new(series: &Series) -> PolarsResult<Self> {
        let r#struct = series.struct_()?;
        let index = r#struct.field_by_name("Index")?;
        let isomerism = r#struct.field_by_name("Isomerism")?;
        let unsaturation = r#struct.field_by_name("Unsaturation")?;
        Ok(Self {
            index,
            isomerism,
            unsaturation,
        })
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn get(&self, index: usize) -> PolarsResult<Unsaturated> {
        Ok(Unsaturated {
            index: self.index.u8()?.get(index),
            isomerism: self
                .isomerism
                .i8()?
                .get(index)
                .and_then(|isomerism| isomerism.try_into().ok()),
            unsaturation: self
                .unsaturation
                .u8()?
                .get(index)
                .and_then(|unsaturation| unsaturation.try_into().ok()),
        })
    }
}
