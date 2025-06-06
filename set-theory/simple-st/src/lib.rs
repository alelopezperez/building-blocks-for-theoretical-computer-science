use std::{collections::BTreeSet, fmt::Display};

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
impl Set {
    pub fn new(elements: Vec<ElemOrSet>) -> Self {
        if elements.is_empty() {
            Set::EmptySet
        } else {
            Set::Set(elements.into_iter().collect::<BTreeSet<_>>())
        }
    }
    pub fn set_cardinality(&self) -> usize {
        match self {
            Set::EmptySet => 0,
            Set::Set(btree_set) => btree_set.len(),
        }
    }

    pub fn is_set_von_nueman_ordinal(&self) -> Option<usize> {
        if let Set::EmptySet = self {
            Some(0)
        } else {
            let mut count = 1;
            while 1 <= self.set_cardinality() {
                if natural_to_set(count) == *self {
                    return Some(count);
                }
                count += 1;
            }
            None
        }
    }

    pub fn big_union(&self) -> Set {
        if let Set::Set(set) = &self {
            let mut union_set = BTreeSet::new();
            for v in set {
                if let ElemOrSet::Set(Set::Set(seti)) = v {
                    for u in seti {
                        union_set.insert(u.clone());
                    }
                }
            }
            Set::Set(union_set)
        } else {
            Set::EmptySet
        }
    }
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
    }
}

pub fn create_ordered_pair(fst: ElemOrSet, snd: ElemOrSet) -> Set {
    todo!()
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

enum Token {
    SmallLetter(char),
    BigLetter(char),
    OBracket,
    CBracket,
    Coma,
    Equal,
}
