use crate::matter::element::Element;
#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]

pub struct Atom {
    element: Element,
    neutrons: u8,
    electrons: u8, // same as protons
}

#[allow(unused)]
impl Atom {
    fn new(element_num: u8) -> Self {
        let element = Element::new(element_num);
        Atom {
            electrons: element.number,
            element,
            neutrons: 0,
        }
    }
}
