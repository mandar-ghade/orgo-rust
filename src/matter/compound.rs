use crate::matter::atom::Atom;

#[derive(Clone, PartialEq, Debug)]
pub struct Compound<'a> {
    // Covalent Compound
    pub center: Atom,
    pub atom_substituents: Vec<Atom>,
    pub compound_substituents: Vec<&'a Compound<'a>>,
}
