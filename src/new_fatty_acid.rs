use std::{
    num::{NonZero, NonZeroU8},
    ops::RangeInclusive,
};
use crate::r#const::relative_atomic_mass::{C, H, O};
use serde::{Deserialize, Serialize};

// pub macro fatty_acid($c:expr $(; $($i:expr),*)*) {{
//     assert!($c > 0);
//     #[allow(unused_mut)]
//     let mut fatty_acid = FattyAcid::new($c);
//     let mut _count = 0;
//     $(
//         _count += 1;
//         $(
//             assert!($i != 0);
//             assert!($i < $c);
//             let r#i8 = ($i as i8);
//             let unsaturated = Unsaturated {
//                 unsaturation: Unsaturation::try_from(_count).ok(),
//                 index: (r#i8 != 0).then_some(r#i8.abs() as _) ,
//                 isomerism: Isomerism::try_from(r#i8).ok(),
//             };
//             fatty_acid.unsaturated.push(unsaturated);
//         )*
//     )*
//     fatty_acid
// }}

pub const C2U0: FattyAcid = FattyAcid::saturated(2..=2);
pub const C4U0: FattyAcid = FattyAcid::saturated(4..=4);
pub const C6U0: FattyAcid = FattyAcid::saturated(6..=6);
pub const C8U0: FattyAcid = FattyAcid::saturated(8..=8);
pub const C10U0: FattyAcid = FattyAcid::saturated(10..=10);
pub const C12U0: FattyAcid = FattyAcid::saturated(12..=12);
pub const C14U0: FattyAcid = FattyAcid::saturated(14..=14);
pub const C16U0: FattyAcid = FattyAcid::saturated(16..=16);
pub const C18U0: FattyAcid = FattyAcid::saturated(18..=18);
pub const C20U0: FattyAcid = FattyAcid::saturated(20..=20);
pub const C22U0: FattyAcid = FattyAcid::saturated(22..=22);
pub const C24U0: FattyAcid = FattyAcid::saturated(24..=24);
pub const C26U0: FattyAcid = FattyAcid::saturated(26..=26);
pub const C28U0: FattyAcid = FattyAcid::saturated(28..=28);
pub const C30U0: FattyAcid = FattyAcid::saturated(30..=30);
pub const C32U0: FattyAcid = FattyAcid::saturated(32..=32);

/// Fatty acid
pub trait FattyAcidExt {
    /// Bounds
    fn b(&self) -> Option<RangeInclusive<u8>> {
        let b = |c| c - 1;
        let c = self.c()?;
        Some(b(c.start())..=b(c.end()))
    }

    /// Hydrogen
    ///
    /// `H = 2C - 2U`
    fn h(&self) -> Option<RangeInclusive<u8>> {
        let h = |c, u| 2 * c - 2 * u;
        let c = self.c()?;
        let u = self.u()?;
        Some(h(c.start(), u.start())..=h(c.end(), u.end()))
    }

    // /// Fatty acid ECN (Equivalent carbon number)
    // ///
    // /// `ECN = C - 2U`
    // fn ecn(&self) -> u8 {
    //     self.c() - 2 * self.u()
    // }

    // /// Mass
    // fn mass(&self) -> f64 {
    //     self.c() as f64 * C + self.h() as f64 * H + 2. * O
    // }

    // /// Saturated
    // fn s(&self) -> bool {
    //     self.u() == 0
    // }

    /// Carbon
    fn c(&self) -> Option<&RangeInclusive<u8>>;

    /// Unsaturated bounds count
    fn u(&self) -> Option<&RangeInclusive<u8>>;
}

/// Fatty acid
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FattyAcid {
    pub carbons: Option<RangeInclusive<u8>>,
    pub unsaturated: Vec<Option<(Option<usize>, Option<u8>)>>,
    // pub unsaturated: Vec<Unsaturated>,
}

impl FattyAcid {
    pub const fn saturated(carbons: RangeInclusive<u8>) -> Self {
        Self {
            carbons: Some(carbons),
            unsaturated: None,
        }
    }

    // fn sort(&mut self) {
    //     self.unsaturated
    //         .sort_by_cached_key(|bound| (bound.unsaturation, bound.isomerism, bound.index));
    // }
}

impl FattyAcidExt for &FattyAcid {
    fn c(&self) -> Option<&RangeInclusive<u8>> {
        self.carbons.as_ref()
    }

    fn u(&self) -> Option<&RangeInclusive<u8>> {
        self.unsaturated.as_ref()
        // self.unsaturated.iter().fold(None, |sum, bound| {
        //     let unsaturation = bound.unsaturation.map(|unsaturation| unsaturation as u8);
        //     match (sum, unsaturation) {
        //         (None, None) => None,
        //         (None, Some(unsaturation)) => Some(unsaturation),
        //         (Some(sum), None) => Some(sum),
        //         (Some(sum), Some(unsaturation)) => Some(sum + unsaturation),
        //     }
        // })
    }
}

/// Unsaturated
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Unsaturated {
    pub index: Option<u8>,
    pub isomerism: Option<Isomerism>,
    pub unsaturation: Option<Unsaturation>,
}

/// Isomerism
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Isomerism {
    Cis = 1,
    Trans = -1,
}

impl From<Isomerism> for i8 {
    fn from(value: Isomerism) -> Self {
        match value {
            Isomerism::Cis => 1,
            Isomerism::Trans => -1,
        }
    }
}

impl TryFrom<i8> for Isomerism {
    type Error = i8;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value.is_positive() {
            Ok(Self::Cis)
        } else if value.is_negative() {
            Ok(Self::Trans)
        } else {
            Err(value)
        }
    }
}

/// Unsaturation
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Unsaturation {
    #[default]
    One = 1,
    Two = 2,
}

impl TryFrom<u8> for Unsaturation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::display::{COMMON, DisplayWithOptions};

    #[test]
    fn test() {
        println!("X: {}", (2..=1).count());
        // let fatty_acid = fatty_acid!(18;9);
        let fatty_acid = FattyAcid::saturated(18..=18);
        for carbons in fatty_acid.carbons.into_iter().flatten() {
            println!("WTF!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {carbons}");
        }
        // println!("{fatty_acid:?}");
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     // #[test]
//     // fn isomerism() {
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,-12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12t15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,-12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12t15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,-12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12t15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12c15t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;-9,-12,-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9t12t15t",
//     //     );
//     //     // 2:1
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15;-9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15;-12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12t",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;-15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15t",
//     //     );
//     //     // 1:2
//     // }

//     // #[test]
//     // fn order() {
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,9,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,9,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,12,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c15c",
//     //     );
//     //     // 2:1
//     //     assert_eq!(
//     //         fatty_acid!(18;12,15;9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,12;9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c15c-9c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,15;12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15,9;12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c15c-12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12,9;15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c12c-15c",
//     //     );
//     //     // 1:2
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c-12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;15,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-9c-12c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12;9,15)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c-9c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;12;15,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-12c-9c15c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15;9,12)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-15c-9c12c",
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;15;12,9)
//     //             .display(Kind::ColonMinus)
//     //             .to_string(),
//     //         "18-15c-9c12c",
//     //     );
//     // }

//     // #[test]
//     // fn macros() {
//     //     // 0
//     //     assert_eq!(fatty_acid!(18), new(vec![0; 17]));
//     //     // 1
//     //     assert_eq!(
//     //         fatty_acid!(18;9),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
//     //     );
//     //     // 2
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;;9,12),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0]),
//     //     );
//     //     // 3
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9,12;15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 2, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;9;12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 2, 0, 0]),
//     //     );
//     //     assert_eq!(
//     //         fatty_acid!(18;;9,12,15),
//     //         FattyAcid::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0]),
//     //     );
//     // }

//     mod errors {
//         use super::*;

//         #[test]
//         #[should_panic(expected = "assertion failed: 0 > 0")]
//         fn zero_carbons() {
//             fatty_acid!(0);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 0 != 0")]
//         fn zero_index() {
//             fatty_acid!(18;0);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 18 < 18")]
//         fn equal_carbons() {
//             fatty_acid!(18;18);
//         }

//         #[test]
//         #[should_panic(expected = "assertion failed: 19 < 18")]
//         fn greater_carbons() {
//             fatty_acid!(18;19);
//         }
//     }

//     #[test]
//     fn common() {
//         let fatty_acid = fatty_acid!(18).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:0");
//         assert_eq!(format!("{fatty_acid:02}"), "18:00");
//         assert_eq!(format!("{fatty_acid:#}"), "18:0");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:00");
//         let fatty_acid = &fatty_acid!(18;9).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:1");
//         assert_eq!(format!("{fatty_acid:02}"), "18:01");
//         assert_eq!(format!("{fatty_acid:#}"), "18:1Δ9");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:01Δ09");
//         let fatty_acid = fatty_acid!(18;9,12).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:2");
//         assert_eq!(format!("{fatty_acid:02}"), "18:02");
//         assert_eq!(format!("{fatty_acid:#}"), "18:2Δ9,12");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:02Δ09,12");
//         // Triple
//         let fatty_acid = fatty_acid!(18;9;12).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:1:1");
//         assert_eq!(format!("{fatty_acid:02}"), "18:01:01");
//         assert_eq!(format!("{fatty_acid:#}"), "18:1:1Δ9,12");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:01:01Δ09,12");
//         // Isomerism
//         let fatty_acid = fatty_acid!(18;-9,-12,-15).display(COMMON);
//         assert_eq!(fatty_acid.to_string(), "18:3");
//         assert_eq!(format!("{fatty_acid:02}"), "18:03");
//         assert_eq!(format!("{fatty_acid:#}"), "18:3Δ9t,12t,15t");
//         assert_eq!(format!("{fatty_acid:#02}"), "18:03Δ09t,12t,15t");
//     }

//     #[test]
//     fn id() {
//         let fatty_acid = fatty_acid!(18).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u0");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u00");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u0");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u00");
//         let fatty_acid = fatty_acid!(18;9).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u1");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u01");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u1c9");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u01c09");
//         let fatty_acid = fatty_acid!(18;9,12).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u2");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u02");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u2c9c12");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u02c09c12");
//         // Triple
//         let fatty_acid = fatty_acid!(18;9;12).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u1u1");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u01u01");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u1u1c9c12");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u01u01c09c12");
//         // Isomerism
//         let fatty_acid = fatty_acid!(18;-9,-12,-15).display(ID);
//         assert_eq!(fatty_acid.to_string(), "c18u3");
//         assert_eq!(format!("{fatty_acid:02}"), "c18u03");
//         assert_eq!(format!("{fatty_acid:#}"), "c18u3t9t12t15");
//         assert_eq!(format!("{fatty_acid:#02}"), "c18u03t09t12t15");
//     }
// }
