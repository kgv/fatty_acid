use super::FattyAcidExpr;
use polars::prelude::*;

/// Fatty acid
pub trait FattyAcidExt {
    /// See [`bounds`]
    fn b(&self) -> Expr;

    /// See [`carbons`]
    fn c(&self) -> Expr;

    /// See [`hydrogens`]
    fn h(&self) -> Expr;

    /// See [`saturated`]
    fn s(&self) -> Expr;

    /// See [`unsaturation`]
    fn u(&self) -> Expr;
}

/// Fatty acid
impl FattyAcidExt for FattyAcidExpr {
    fn b(&self) -> Expr {
        self.bounds()
    }

    fn c(&self) -> Expr {
        self.carbons()
    }

    fn h(&self) -> Expr {
        self.hydrogens()
    }

    fn s(&self) -> Expr {
        self.saturated()
    }

    fn u(&self) -> Expr {
        self.unsaturation()
    }
}
