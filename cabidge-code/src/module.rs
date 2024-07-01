use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::func::Function;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module<Sym> {
    pub module_imports: Vec<PathBuf>,

    pub atom_imports: Vec<Import<Sym>>,
    pub atom_defs: Vec<Atom<Sym>>,
    /// Atoms are namespaced but types are universal.
    /// As a result types cannot be imported or exported
    pub type_defs: Vec<TypeDesc>,
    /// Values are allowed to be polymorphic bc they can be functions
    pub value_imports: Vec<Import<Sym>>,
    pub value_defs: Vec<Function>,
    pub value_exports: Vec<Export<Sym>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import<Sym> {
    pub module: usize,
    pub item_name: Sym,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reference {
    pub item: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export<Sym> {
    pub rf: Reference,
    pub name: Sym,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Atom<Sym> {
    /// All atoms are exported
    pub name: Sym,
    pub num_members: usize,
}

/// Type of a function or type argument/param
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgType {
    Concrete(Reference),
    Generic(u32),
}

/// A type
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeDesc {
    /// References here are to atoms
    AtomGroup(Vec<Adapted<Reference>>),
    Func {
        input: ArgType,
        /// Must either be another func or BottomThunk
        output: ArgType,
    },
    /// References here are to TypeDescs
    Tuple(Vec<Adapted<Reference>>),
    /// A computation that never returns
    BottomThunk,
    Int8,
    Int16,
    Int32,
    Int64,
    //Float16??
    Float32,
    Float64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adapted<T> {
    pub params: Vec<ArgType>,
    pub item: T,
}
