use std::fmt;

use super::*;

impl APG {
    pub fn one(name: Rc<Name>, element_index: Index) -> APG {
        let mut elements = IndexSet::new();
        elements.insert(element_index);
        let lambda = vec![0];
        let upsilon = vec![0];

        APG {
            name: name,
            elements: elements,
            lambda: lambda,
            upsilon: upsilon,
        }
    }
}

impl fmt::Debug for APG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "[{}]", self.name);
        let _ = writeln!(f, "elements: {:?}", self.elements);
        let _ = writeln!(f, "lambda:   {:?}", self.lambda);
        writeln!(f, "upsilon:  {:?}", self.upsilon)
    }
}