use serde::{Deserialize, Serialize};

use crate::{err::Bounds, module::{Adapted, Reference}};

/// Functions take in arguments, run a sequence of [Op]s (mainly [Op::Apply]) to create potential computations,
/// and then branches on an atom, executing the corresponding computation and throwing out all others.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub args: Vec<Arg>,
    // no need for a return type. it is always a BottomThunk
    pub defs: Vec<Def>,
    pub term: Terminal
}

/// An operation guaranteed to terminate in bounded time (this is why functions do not run when finished applying)
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Op {
    LoadExtern (Adapted<Reference>),
    LoadAtom(Adapted<Reference>),
    Apply {
        func: Value,
        input: Value,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Value(pub usize);

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arg {
    pub val: Value,
    pub type_ref: Reference,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Def {
    pub dec: Value,
    pub op: Op,
    pub loc: Bounds,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Terminal {
    /// Execute a thunk
    Exec(Value),
    /// match statement
    Match(Vec<MatchArm>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchArm {
    pub atom: Reference,
    /// arguments in the atom are passed to this function
    pub branch: Value
}