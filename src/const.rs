pub(crate) mod atoms {
    use atom::isotopes::*;

    pub(crate) const C: C = C::Twelve;
    pub(crate) const H: H = H::One;
    pub(crate) const O: O = O::Sixteen;
}

pub(crate) mod relative_atomic_mass {
    use atom::isotopes::*;

    pub(crate) const C: f64 = C::Twelve.relative_atomic_mass().value;
    pub(crate) const H: f64 = H::One.relative_atomic_mass().value;
    pub(crate) const LI: f64 = Li::Seven.relative_atomic_mass().value;
    pub(crate) const N: f64 = N::Fourteen.relative_atomic_mass().value;
    pub(crate) const NA: f64 = Na.relative_atomic_mass().value;
    pub(crate) const O: f64 = O::Sixteen.relative_atomic_mass().value;
}
