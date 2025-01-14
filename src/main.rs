//use thiserror::Error;

use std::{
    borrow::{Borrow, BorrowMut},
    cell::{BorrowMutError, RefCell},
    collections::{linked_list, HashMap, HashSet, LinkedList},
    fmt::Debug,
    iter::Peekable,
    ops::Index,
    path::Iter,
    rc::Rc,
};

use strum_macros::Display;
use thiserror::Error;

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

#[allow(unused)]
#[derive(Clone, PartialEq)]
struct Compound {
    // Atom chain. Composite data-structure of individual `Compound` fragments,
    // abstraction of an Organic Compound in chemistry
    center: Atom,
    substituents: Vec<Particle>, // substituents
}

impl Debug for Compound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Compound")
            .field("center", &self.center)
            .field("substituents", &self.substituents)
            .finish()
    }
}

impl Compound {
    // Creates a `Compound` with initial center `Atom`
    fn builder() -> CompoundBuilder {
        CompoundBuilder::default()
    }

    fn new(center: Atom) -> Self {
        Self {
            center,
            substituents: Vec::new(),
        }
    }

    fn condensed_formula(&self) -> String {
        todo!("Condensed formula fetching not implemented")
    }
}

struct Compounds {
    chain: LinkedList<Compound>,
}

impl IntoIterator for Compounds {
    type Item = Compound;
    type IntoIter = linked_list::IntoIter<Compound>;
    fn into_iter(self) -> Self::IntoIter {
        self.chain.into_iter()
    }
}

#[derive(Default)]
#[allow(unused)]
struct CompoundBuilder {
    data: HashMap<u8, Vec<u8>>,
    all: Vec<Compound>,
    parent_len: u8,
    curr: u8,
}

trait Builder: Sized {
    type Err;
    /// chains `n` carbons linearly
    fn chain(self, n: u8) -> Self;
    /// Chains `n` carbons at a locant
    fn chain_at(self, n: u8) -> Self;
    /// Returns # of chains connected to itself
    fn chain_len(&self) -> u8;
    // Finds longest chain and converts that into a LinkedList
    fn to_linked_list(&self) -> LinkedList<Compound>;
    /// Builds Compound once operations have been completed.
    fn build(&self) -> Result<Compounds, Self::Err>;
}

impl Builder for CompoundBuilder {
    type Err = CompoundUsageError;

    fn chain(self, n: u8) -> Self {
        todo!()
    }

    fn chain_at(self, n: u8) -> Self {
        todo!()
    }

    fn chain_len(&self) -> u8 {
        todo!()
    }

    fn to_linked_list(&self) -> LinkedList<Compound> {
        // non-public
        todo!();
    }

    fn build(&self) -> Result<Compounds, Self::Err> {
        Ok(Compounds {
            chain: self.to_linked_list(),
        })
    }
}

impl CompoundBuilder {
    fn new() -> Self {
        Self::default()
    }
}

fn test_compound() -> Result<Rc<RefCell<Compound>>, CompoundUsageError> {
    todo!("Example code is unfinished");
}

fn main() {
    dbg!(test_compound());
}
