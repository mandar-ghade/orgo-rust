//use thiserror::Error;

use std::{borrow::Borrow, iter::Peekable, slice::Iter};

use strum_macros::Display;
use thiserror::Error;

enum Chirality {
    R,
    S,
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct Element {
    number: u8,
}

impl Element {
    fn new(number: u8) -> Self {
        Element { number }
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct Atom {
    element: Element,
    oxi: Option<u8>, // oxidation state
}

impl Atom {
    fn new(element: Element, oxi: Option<u8>) -> Self {
        Atom { element, oxi }
    }
}

#[derive(Error, Clone, Debug, Display)]
enum ParsingError {
    MoleculeParsingError(String),
    CompoundParsingError(String),
}

#[allow(unused)]
#[derive(Clone, Debug)]
enum Particle {
    Atom(Atom),
    Molecule(Vec<Atom>),
}

impl Particle {
    fn new_atom(element: Element, oxi: Option<u8>) -> Particle {
        Particle::Atom(Atom::new(element, oxi))
    }

    fn new_molecule(atoms: Vec<Atom>) -> Result<Particle, ParsingError> {
        Ok(Particle::Molecule(atoms)).map_err(ParsingError::MoleculeParsingError)
    }
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct Compound {
    center: Atom,
    subst: Vec<Particle>, // substituents
    side_chains: Vec<Box<Compound>>,
}

impl Compound {
    fn as_box(
        center: Atom,
        subst: Vec<Particle>,
        side_chains: Vec<Box<Compound>>,
    ) -> Result<Box<Compound>, ParsingError> {
        Ok(Box::new(Compound {
            center,
            subst,
            side_chains,
        }))
        .map_err(ParsingError::CompoundParsingError)
    }
}

trait Parsable {}

fn main() {
    let mut cmp = Compound::as_box(Atom::new(Element::new(3), None), vec![], vec![])
        .expect("Compound was expected");
    cmp.subst.push(Particle::new_atom(Element::new(18), None));
    cmp.subst.push(
        Particle::new_molecule(vec![
            Atom::new(Element::new(7), None),
            Atom::new(Element::new(7), None),
        ])
        .expect("Molecule not parsed"),
    );
    let cmp2 = Compound::as_box(Atom::new(Element::new(3), None), vec![], vec![cmp]);
    dbg!(&cmp2);
}
