use serde::{Deserialize, Serialize};

use crate::module::{Atom, Reference, TypeDesc, Use};

/// Functions take in arguments, run a sequence of [Op]s (mainly [Op::Apply]) to create potential computations,
/// and then branches on an atom, executing the corresponding computation and throwing out all others.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub args: Vec<Arg>,
    // no need for a return type. it is always a BottomThunk
    pub def: Vec<Def>,
    pub term: Terminal<Use<Atom>>,
}

/// An operation guaranteed to terminate in bounded time (this is why functions do not run when finished applying)
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    LoadFunc(Reference<Function>),
    LoadAtom { atom_ref: Reference<Atom> },
    Apply { func: Value, inputs: Vec<Value> },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value(pub usize);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arg {
    pub val: Value,
    pub type_ref: Synced<Use<TypeDesc>>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Def {
    pub dec: Value,
    pub op: Op,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Terminal<A> {
    /// Execute a thunk
    Exec(Value),
    /// Atom match statement
    MatchAtom(Match<Reference<A>>),
    /// Number match statement
    MatchNum(Match<u32>)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchArm<A> {
    pub val: A,
    /// arguments in the atom are passed to this function
    pub branch: Value,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match<A> {
    pub arms: Vec<MatchArm<A>>,
    pub default: Value
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Synced<T> {
    pub adjusted_generics: Vec<u32>,
    pub desc: Reference<T>
}