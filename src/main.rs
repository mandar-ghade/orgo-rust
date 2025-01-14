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
struct CompoundBuilder {
    data: HashMap<u8, Vec<u8>>,
    parent: u8,
    curr: u8,
}

trait Builder: Sized {
    type Err;
    fn push(&mut self, p: Particle) -> Self;
    /// Appends a Compound to itself
    fn append(&mut self, chain: Compound) -> Self;
    /// Pops last chain
    fn pop(&mut self) -> Self;
    /// Gets first appended chain
    fn first_chain(&self) -> Self;
    /// Gets last appended chain
    fn last_chain(&self) -> Self;
    /// Goes to previous compound
    fn super_chain(&self) -> Self;
    /// Returns # of chains connected to itself
    fn chain_len(&self) -> u8;
    /// Builds Compound once operations have been completed.
    fn build(&self) -> Result<Compounds, Self::Err>;
}

impl Builder for CompoundBuilder {
    type Err = CompoundUsageError;
    fn push(&mut self, p: Particle) -> Self {
        todo!("Pushing not implemented")
    }

    fn append(&mut self, chain: Compound) -> Self {
        todo!("Appending not implemented")
    }

    fn pop(&mut self) -> Self {
        todo!("Compound chain pop not implemented")
    }

    fn last_chain(&self) -> Self {
        todo!("Last chain not implemented")
    }

    fn first_chain(&self) -> Self {
        todo!("Get first connecting chain not implemented")
    }

    fn super_chain(&self) -> Self {
        todo!("Super chain not implemented")
    }

    fn chain_len(&self) -> u8 {
        todo!("Chain lengths not implemented")
    }

    fn build(&self) -> Result<Compounds, Self::Err> {
        todo!("Compound building not implemented")
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
