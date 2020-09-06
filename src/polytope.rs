mod apg;


use std::rc::Rc;
use std::collections::{HashMap, BTreeSet};
use indexmap::set::IndexSet;
use std::fmt;

pub use apg::*;


type Element = usize;
type Name = String;

#[derive(PartialEq, Eq, Hash, Default, Debug)]
struct Label(Rc<Name>);

#[derive(PartialEq, Eq, Hash, Debug)]
enum Value {
    Unit,
}

type Index = usize;
type LabelIndex = Index;
type ValueIndex = Index;

#[derive(Default)]
pub struct APG {
    name: Rc<Name>,
    elements: IndexSet<Element>,
    lambda: Vec<ValueIndex>,
    upsilon: Vec<LabelIndex>,
}

pub struct APGMorphism {
    element_mapping: Box<dyn Fn(&Element) -> Element>,
    label_mapping: Box<dyn Fn(&LabelIndex) -> LabelIndex>,
    value_mapping: Box<dyn Fn(&ValueIndex) -> ValueIndex>,
}

impl Default for APGMorphism {
    fn default() -> Self {
        APGMorphism {
            element_mapping: Box::new(|e| e.clone()),
            label_mapping: Box::new(|l| l.clone()),
            value_mapping: Box::new(|v| v.clone()),
        }
    }
}

impl APGMorphism {
    pub fn default_coproduct(label_index: LabelIndex) -> Self {
        APGMorphism {
            element_mapping: Box::new(|e| e.clone()),
            label_mapping: Box::new(move |l| {
                if l == &0 {
                    label_index
                } else {
                    l.clone()
                }
            }),
            value_mapping: Box::new(|v| v.clone()),
        }
    }
}

#[derive(Default)]
pub struct Polytope {
    next_element_index: Index,
    names: BTreeSet<Rc<Name>>,

    labels: IndexSet<Label>,
    values: IndexSet<Value>,

    apgs: HashMap<Name, Rc<APG>>,
}

impl Polytope {
    pub fn new() -> Polytope {
        let mut labels = IndexSet::new();
        labels.insert(Label::default());
        let mut values = IndexSet::new();
        values.insert(Value::Unit);

        Polytope {
            next_element_index: 0,
            names: BTreeSet::new(),

            labels: labels,
            values: values,
            apgs: HashMap::new(),
        }
    }

    pub fn make_one(&mut self, name: &str) {
        let key = Rc::new(name.to_string());
        if !self.names.contains(&key) {
            self.names.insert(key.clone());
        }

        let n = self.names.get(&key).unwrap();
        let element_index = self.next_element_index;
        self.next_element_index += 1;

        self.apgs.insert(name.to_string(), Rc::new(APG::one(n.clone(), element_index)));
    }

    pub fn get_apg(&self, name: &str) -> Rc<APG> {
        self.apgs.get(name).unwrap().clone()
    }

    pub fn make_coproduct(&mut self, name: &str, left: Rc<APG>, right: Rc<APG>, inl: APGMorphism, inr: APGMorphism) {
        let mut elements = IndexSet::new();
        elements.extend(left.elements.clone().iter().map(inl.element_mapping).collect::<IndexSet<_>>());
        elements.extend(right.elements.clone().iter().map(inr.element_mapping).collect::<IndexSet<_>>());

        let lambda = [
            left.lambda.clone().iter().map(inl.label_mapping).collect::<Vec<_>>(), 
            right.lambda.clone().iter().map(inr.label_mapping).collect::<Vec<_>>(),
        ].concat();
        let upsilon = [
            left.upsilon.clone().iter().map(inl.value_mapping).collect::<Vec<_>>(), 
            right.upsilon.clone().iter().map(inr.value_mapping).collect::<Vec<_>>(),
        ].concat();

        let apg = APG {
            name: Rc::new(name.to_string()),
            elements: elements,
            lambda: lambda,
            upsilon: upsilon,
        };
        self.apgs.insert(name.to_string(), Rc::new(apg));
    }

    pub fn make_morphism_coproduct(&mut self, apg: Rc<APG>) -> APGMorphism {
        let name = Label(apg.name.clone());
        if let Some(label_index) = self.labels.get_index_of(&name) {
            APGMorphism::default_coproduct(label_index)
        } else {
            let new_label = Label(apg.name.clone());
            self.labels.insert(new_label);
            let new_label_index = self.labels.get_index_of(&name);
            APGMorphism::default_coproduct(new_label_index.unwrap())
        }
    }
}

impl fmt::Debug for Polytope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = writeln!(f, "next_element_index: {:?}", self.next_element_index);
        let _ = writeln!(f, "names:   {:?}", self.names);
        let _ = writeln!(f, "labels:  {:?}", self.labels);
        let _ = writeln!(f, "values:  {:?}", self.values);
        let _ = writeln!(f, "apg:     {}", "{");
        for apg in self.apgs.values() {
            let _ = writeln!(f, "{:?}", apg);
        }
        writeln!(f, "{}", "}")
    }
}