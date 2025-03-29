use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
};

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
pub enum Set {
    EmptySet,
    Set(BTreeSet<ElemOrSet>),
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
pub enum ElemOrSet {
    Symbol(char),
    Set(Set),
}

impl Display for ElemOrSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElemOrSet::Symbol(sym) => write!(f, "{}", sym),
            ElemOrSet::Set(set) => write!(f, "{}", set),
        }
    }
}
impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Set::EmptySet => write!(f, "∅"),
            Set::Set(btree_set) => {
                let mut st = String::new();
                st.push_str(" {");
                for v in btree_set {
                    let fo = format!("{}", v);
                    st += &fo;
                }

                st.push_str("} ");
                write!(f, "{st}")
            }
        }
    }
}

pub fn succ(set: Set) -> Set {
    match set {
        Set::EmptySet => Set::Set(BTreeSet::from([ElemOrSet::Set(Set::EmptySet)])),
        Set::Set(btree_set) => {
            let mut cp_bt = btree_set.clone();
            cp_bt.insert(ElemOrSet::Set(Set::Set(btree_set)));
            Set::Set(cp_bt)
        }
        _ => {
            panic!("You can only succ a Number")
        }
    }
}

pub fn natural_to_set(num: usize) -> Set {
    (0..num).fold(Set::EmptySet, |acc, _| succ(acc))
}

pub fn union_op(set_a: Set, set_b: Set) -> Set {
    match (set_a, set_b) {
        (Set::EmptySet, Set::EmptySet) => Set::EmptySet,
        (Set::EmptySet, Set::Set(btree_set)) => Set::Set(btree_set),
        (Set::Set(btree_set), Set::EmptySet) => Set::Set(btree_set),
        (Set::Set(a), Set::Set(b)) => {
            let mut union = a;
            for b in b {
                union.insert(b);
            }
            Set::Set(union)
        }
    }
}

fn create_set(values: Vec<char>) -> Set {
    todo!()
}

pub fn ordered_pair() {}
