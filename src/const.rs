pub(crate) mod relative_atomic_mass {
    use atom::isotopes::*;

    pub(crate) const C: f64 = C::Twelve.relative_atomic_mass().value;
    pub(crate) const H: f64 = H::One.relative_atomic_mass().value;
    pub(crate) const O: f64 = O::Sixteen.relative_atomic_mass().value;
}
