//use thiserror::Error;

use std::{
    borrow::{Borrow, BorrowMut},
    cell::{BorrowMutError, RefCell},
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::Index,
    path::Iter,
    rc::Rc,
};

use strum_macros::Display;
use thiserror::Error;

enum Chirality {
    R,
    S,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
struct Element {
    number: u8,
}

impl Element {
    fn new(number: u8) -> Self {
        Element { number }
    }
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
struct Atom {
    element: Element,
    neutrons: u8,
    electrons: u8,   // same as protons
    oxi: Option<u8>, // oxidation state
}

impl Atom {
    fn new(element_num: u8) -> Self {
        let element = Element::new(element_num);
        Atom {
            electrons: element.number,
            element,
            neutrons: 0,
            oxi: None,
        }
    }
}

#[derive(Error, Clone, Debug, Display)]
enum ParsingError {
    MoleculeParsingError(String),
    CompoundParsingError(String),
}

#[derive(Error, Clone, Debug, Display)]
enum CompoundUsageError {
    SubstituentPushingError(String),
    CompoundPushingError(String),
    ParsingError(ParsingError),
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
enum Particle {
    Atom(Atom),
    Molecule(Vec<Atom>),
}

impl Particle {
    fn new_atom(atom: Atom) -> Particle {
        Particle::Atom(atom)
    }

    fn new_molecule(atoms: Vec<Atom>) -> Particle {
        Particle::Molecule(atoms)
    }
}

struct AtomCounter {
    atoms: HashMap<u8, u8>,
}

impl ToString for AtomCounter {
    fn to_string(&self) -> String {
        todo!()
    }
}

// utilize compound iterable class
impl AtomCounter {
    fn from_compound(cmp: &Compound) -> Self {
        todo!()
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq)]
struct Compound {
    // Atom chain. Composite data-structure of individual `Compound` fragments,
    // abstraction of an Organic Compound in chemistry, but heavily convoluted
    center: Atom,
    subst: Vec<Particle>,                    // substituents
    side_chains: Vec<Rc<RefCell<Compound>>>, // previous compounds
}

impl Debug for Compound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Compound")
            .field("center", &self.center)
            .field("subst", &self.subst)
            .field("side_chains_count", &self.side_chains.len())
            .finish()
    }
}

impl Compound {
    // Creates a `Compound` with initial center `Atom`
    fn new(center: Atom) -> Self {
        Self {
            center,
            subst: vec![],
            side_chains: vec![],
        }
    }

    fn with_center(center: Atom) -> Self {
        Self {
            center,
            subst: vec![],
            side_chains: vec![],
        }
    }

    fn as_rc_with_center(center: Atom) -> Rc<RefCell<Compound>> {
        Self::with_center(center).rc()
    }

    fn condensed_formula(&self) -> String {
        AtomCounter::from_compound(self).to_string()
    }
}

trait ToRc {
    fn rc(self) -> Rc<RefCell<Self>>;
}

impl ToRc for Compound {
    // Converts Compound to `Rc` so operations can be done on it
    fn rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}

trait CompoundModifier: Sized {
    // size of `Self` must be known at compile time
    type Err;
    fn add_substituent(&self, p: Particle) -> Result<Self, Self::Err>;
    fn add_side_chain(&self, chain: Rc<RefCell<Compound>>) -> Result<Self, Self::Err>;
}

// Iterator of Compounds
struct Compounds {
    data: HashMap<Rc<RefCell<Compound>>, Vec<Rc<RefCell<Compound>>>>,
    // HASHMAP!!!!!!!!!
}

// trait CompoundIterable {
//     type Err;
//     fn next_chain(&self) -> Result<Rc<RefCell<Compound>>, Self::Err>;
//     fn next_substituent(&self) -> Result<&Particle, Self::Err>;
// }

impl CompoundModifier for Rc<RefCell<Compound>> {
    type Err = CompoundUsageError;

    fn add_substituent(&self, p: Particle) -> Result<Self, Self::Err> {
        (*self)
            .try_borrow_mut()
            .map_err(|err| CompoundUsageError::SubstituentPushingError(err.to_string()))?
            .subst
            .push(p);
        Ok(Rc::clone(self))
    }
    /// Adds a side chain to a `Compound`
    /// (Extends a `Compound` using an existing chain)
    ///
    /// # Arguments
    /// * `chain` - side_chain which `Compound` is directly attached to now.
    ///
    /// Returns
    ///
    /// The resulting compound once `chain`  been pushed to `Self`.
    fn add_side_chain(&self, chain: Rc<RefCell<Compound>>) -> Result<Self, Self::Err> {
        let same_chain = self.as_ptr() == chain.as_ptr();
        let borrowed = (*self).try_borrow(); // immut
        let vec_contains_chain =
            borrowed.is_ok_and(|b| b.side_chains.iter().any(|c| c.as_ptr() == chain.as_ptr()));
        if !same_chain && !vec_contains_chain {
            (*self)
                .try_borrow_mut()
                .map_err(|e| CompoundUsageError::CompoundPushingError(e.to_string()))?
                .side_chains
                .push(Rc::clone(&chain)); // mut
            chain.add_side_chain(Rc::clone(self))?; // preserves bi-directionality of compounds
        }
        Ok(Rc::clone(self))
    }
}

fn test_compound() -> Result<Rc<RefCell<Compound>>, CompoundUsageError> {
    let compound = Compound::with_center(Atom::new(3))
        .rc()
        .add_substituent(Particle::new_atom(Atom::new(18)))?
        .add_substituent(Particle::new_molecule(vec![Atom::new(7), Atom::new(7)]))?;
    let compound2 = Compound::as_rc_with_center(Atom::new(4)).add_side_chain(Rc::clone(&compound));
    // Self referential side chain appending doesn't work
    // Ex:
    // cmp.add_side_chain(Rc::clone(&cmp))?;
    Ok(compound)
}

fn main() {
    dbg!(test_compound());
}
