use crate::r#const::relative_atomic_mass::{C, H, O};
use polars::prelude::*;

/// Extension methods for [`Expr`]
pub trait ExprExt {
    fn fatty_acid(self) -> FattyAcidExpr;
}

impl ExprExt for Expr {
    fn fatty_acid(self) -> FattyAcidExpr {
        FattyAcidExpr(self)
    }
}

/// Fatty acid [`Expr`]
#[derive(Clone, Debug)]
pub struct FattyAcidExpr(Expr);

impl From<FattyAcidExpr> for Expr {
    fn from(value: FattyAcidExpr) -> Self {
        value.0
    }
}

impl FattyAcidExpr {
    /// Carbons
    pub fn carbons(&self) -> Expr {
        self.0.clone().struct_().field_by_name("Carbons")
    }

    /// Unsaturated
    ///
    /// The number of unsaturated bonds.
    pub fn unsaturated(&self) -> Expr {
        self.0
            .clone()
            .struct_()
            .field_by_name("Unsaturated")
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .len()
    }

    /// Unsaturation
    pub fn unsaturation(&self) -> Expr {
        self.0
            .clone()
            .struct_()
            .field_by_name("Unsaturated")
            .list()
            .eval(col("").struct_().field_by_name("Unsaturation"), true)
            .list()
            .sum()
    }
    // /// Double bounds count
    // pub fn d(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Doubles")
    //         .list()
    //         .len()
    // }

    // /// Triple bounds count
    // pub fn t(&self) -> Expr {
    //     self.0
    //         .clone()
    //         .struct_()
    //         .field_by_name("Triples")
    //         .list()
    //         .len()
    // }

    // pub fn r#type(self) -> Expr {
    //     ternary_expr(self.saturated(), lit("S"), lit("U"))
    // }

    // pub fn unsaturated(self) -> Expr {
    //     self.saturated().not()
    // }

    // pub fn unsaturation(self) -> Expr {
    //     self.d() + lit(2) * self.t()
    // }
}

impl FattyAcidExpr {
    /// Bounds
    pub fn bounds(&self) -> Expr {
        (self.carbons() - lit(1)).clip_min(lit(0))
    }

    /// ECN (Equivalent carbon number)
    ///
    /// `ECN = C - 2U`
    pub fn ecn(&self) -> Expr {
        self.carbons() - lit(2) * self.unsaturation()
    }

    /// Hydrogens
    ///
    /// `H = 2C - 2U`
    pub fn hydrogens(&self) -> Expr {
        lit(2) * self.carbons() - lit(2) * self.unsaturation()
    }

    /// Mass
    pub fn mass(&self) -> Expr {
        self.carbons() * lit(C) + self.hydrogens() * lit(H) + lit(2) * lit(O)
    }

    /// Saturated
    pub fn saturated(&self) -> Expr {
        self.unsaturation().eq(0)
    }

    /// [`bounds`]
    pub fn b(&self) -> Expr {
        self.bounds()
    }

    /// [`carbons`]
    pub fn c(&self) -> Expr {
        self.carbons()
    }

    /// [`hydrogens`]
    pub fn h(&self) -> Expr {
        self.hydrogens()
    }

    /// [`saturated`]
    pub fn s(&self) -> Expr {
        self.saturated()
    }

    /// [`unsaturation`]
    pub fn u(&self) -> Expr {
        self.unsaturation()
    }
}

pub mod filter;
pub mod find;
pub mod short;
