use super::{fatty_acid::FattyAcidExt, find::Find};
use crate::fatty_acid::polars::expr::find::FindByName;
use polars::prelude::*;

// SFA, ∑MUFA, ∑PUFA, ∑n-6 PUFA, ∑n-3 PUFA, and n-6 PUFA/n-3 PUFA
// col("FA").fa().ufa(col("Value"))
/// Fatty acid indices
pub trait Filter: FattyAcidExt
where
    Self: Sized,
{
    /// SFA
    ///
    /// All saturated fatty acids
    fn sfa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().eq(0))
    }

    /// UFA
    ///
    /// All unsaturated fatty acids
    fn ufa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().neq(0))
    }

    /// MUFA
    ///
    /// All unsaturated fatty acids having only one unsaturated bond.
    fn mufa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().eq(1))
    }

    /// PUFA
    ///
    /// All unsaturated fatty acids having more than one unsaturated bond.
    fn pufa(&self, expr: Expr) -> Expr {
        expr.filter(self.unsaturated().gt(1))
    }

    /// PUFA n-3, n-6, ...
    fn pufan(&self, n: u8) -> Expr {
        self.unsaturated().list().eval(
            col("")
                .struct_()
                .field_by_name("Index")
                .eq(self.carbons() - lit(n)),
            true,
        )
    }

    /// IA (Index of atherogenicity)
    /// (C12:0 + 4 * C14:0 + C16:0) / ΣUFA
    fn ia(&self, expr: Expr) -> Expr {
        (self.c12u0(expr.clone()) + lit(4) * self.c14u0(expr.clone()) + self.c16u0(expr.clone()))
            / self.ufa(expr)
    }

    /// IT (Index of thrombogenicity)
    /// (C14:0 + C16:0 + C18:0)/((0.5*ΣMUFA) + (0.5*ΣPUFAN6) + (3*ΣPUFAN3) + (n-3/n-6)]
    fn it(&self, expr: Expr) -> Expr {
        (self.c14u0(expr.clone()) + self.c16u0(expr.clone()) + self.c18u0(expr.clone()))
            / (lit(0.5) * self.ufa(expr) + lit(0.5))
    }

    /// HH (Hypocholesterolemic/hypercholesterolemic ratio)
    /// (cis-C18:1 + ΣPUFA)/(C12:0 + C14:0 + C16:0)
    fn hh(&self, expr: Expr) -> Expr {
        (self.c18u1(expr.clone()) + self.pufa(expr.clone()))
            / (self.c12u0(expr.clone()) + self.c14u0(expr.clone()) + self.c16u0(expr))
    }

    /// HPI (Health-promoting index)
    /// ΣUFA / (C12:0 + 4 * C14:0 + C16:0)
    fn hpi(&self, expr: Expr) -> Expr {
        self.ufa(expr.clone())
            / (self.c12u0(expr.clone()) + lit(4) * self.c14u0(expr.clone()) + self.c16u0(expr))
    }

    /// UI (Unsaturation index)
    /// 1 * (% monoenoics) + 2 * (%dienoics) + 3 * (% trienoics) + 4 *(% tetraenoics) + 5 * (%pentaenoics) + 6 * (% hexaenoics)
    fn ui(&self, expr: Expr) -> Expr {
        lit("ХУЕТА")
    }

    /// FLQ (Fish lipid quality/flesh lipid quality)
    /// (EPA + DHA) / ΣFA
    fn flq(&self, expr: Expr) -> Expr {
        (self.epa(expr.clone()) + self.dha(expr.clone())) / expr.sum()
    }

    /// TFA (Trans fatty acid)
    ///
    fn tfa(&self, expr: Expr) -> Expr {
        expr.filter(self.carbons().eq(18).and(self.unsaturated().eq(3)))
    }
}

impl<T: FattyAcidExt> Filter for T {}
