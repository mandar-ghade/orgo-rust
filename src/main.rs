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
    fn builder() -> OrganicCompoundBuilder {
        OrganicCompoundBuilder::default()
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

enum Compounds {
    Compound(Compound),
    Chain(LinkedList<Compounds>),
}

impl Compounds {
    fn flatten(&self) -> Vec<Compound> {
        let mut all: Vec<Compound> = Vec::new();
        match self {
            Self::Compound(c) => all.push(c.clone()),
            Self::Chain(c) => {
                for item in c.iter() {
                    all.extend(item.flatten())
                }
            }
        }
        all
    }
}

impl IntoIterator for Compounds {
    type Item = Compound;
    type IntoIter = std::vec::IntoIter<Compound>;
    fn into_iter(self) -> Self::IntoIter {
        self.flatten().into_iter()
    }
}

#[derive(Default)]
#[allow(unused)]
struct OrganicCompoundBuilder {
    data: HashMap<u8, Vec<u8>>,
    all: Vec<Compound>,
    parent_chain: u8,
    curr_size: u8,
}

#[allow(unused)]
trait Builder: Sized {
    type Err;
    /// chains `n` carbons linearly
    fn chain(self, n: u8) -> Self;
    /// Chains `n` carbons at a locant
    fn chain_at(self, locant: u8, n: u8) -> Self;
    /// Returns # of chains connected to itself
    fn chain_len(&self) -> u8;
    // Converts chain into LinkedList
    fn to_linked_list(&self) -> LinkedList<Compounds>;
    /// Builds Compound once operations have been completed.
    fn build(self) -> Compounds;
}

impl OrganicCompoundBuilder {
    fn fetch_nodes(&self, i: u8) -> Vec<&u8> {
        todo!()
    }
    fn empty_parent_chain(&self) -> bool {
        self.parent_chain == 0
    }
    fn idx_exists(&self) -> bool {
        true
    }
    fn chain_exists(&self) -> bool {
        true
    }
}

impl Builder for OrganicCompoundBuilder {
    type Err = CompoundUsageError;

    fn chain(mut self, n: u8) -> Self {
        if self.parent_chain == 0 {
            self.parent_chain = n;
        }
        let size = self.curr_size;
        for i in size..size + n {
            self.data.entry(i).or_default();
        }
        self.curr_size += n;
        self
    }

    fn chain_at(mut self, locant: u8, size: u8) -> Self {
        let idx = locant - 1; // n is locant, so idx is 1 less
        if self.empty_parent_chain() {
            panic!("Parent chain not specified")
        } else if idx > self.parent_chain || !self.data.contains_key(&idx) {
            panic!("Parent chain not identified");
        }
        let start_idx = self.curr_size;
        self.curr_size += size;
        self.data
            .get_mut(&idx)
            .expect("Expected mutable parent compound")
            .push(start_idx);
        self
    }

    fn chain_len(&self) -> u8 {
        self.parent_chain
    }

    fn to_linked_list(&self) -> LinkedList<Compounds> {
        // non-public
        todo!();
    }

    fn build(self) -> Compounds {
        if self.curr_size == 1 {
            return Compounds::Compound(
                self.all
                    .first()
                    .expect("Should've gotten first Element")
                    .clone(),
            );
        }
        Compounds::Chain(self.to_linked_list())
    }
}

impl OrganicCompoundBuilder {
    fn new() -> Self {
        Self::default()
    }
}

fn test_compound() -> Compounds {
    OrganicCompoundBuilder::new().chain(6).build()
}

fn main() {
    test_compound();
}
