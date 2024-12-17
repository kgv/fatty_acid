use super::fatty_acid::FattyAcidExt;
use polars::prelude::*;

/// Find fatty acid
pub trait Find: FindByName {
    fn c12u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(12).and(self.unsaturated().eq(0)))
    }

    fn c14u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(14).and(self.unsaturated().eq(0)))
    }

    fn c16u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(16).and(self.unsaturated().eq(0)))
    }

    fn c18u0(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(0)))
    }

    fn c18u1(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(1)))
    }
}

// impl<T: Find> FindByName for T {}

// impl<T: FattyAcidExt> Find for T {}

pub trait FindByName: FattyAcidExt {
    /// EPA (Eicosapentaenoic acid)
    /// C22:6 n-3
    fn epa(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(22).and(self.unsaturated().eq(6)))
    }

    /// DHA (Docosahexaenoic acid)
    /// C20:5 n-3
    fn dha(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(20).and(self.unsaturated().eq(5)))
    }

    /// LA (Linoleic acid)
    /// C18:2 n-6
    fn la(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(2)))
    }

    /// ALA (α-Linolenic acid)
    /// C18:3 n-3
    fn ala(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(3)))
    }
}

impl<T: FattyAcidExt> FindByName for T {}
