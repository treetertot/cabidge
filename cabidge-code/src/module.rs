use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::func::Function;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module<Sym> {
    pub module_imports: Vec<PathBuf>,

    /// Atoms and types are both universal and tehrefore not namespaced.
    /// This is for easy interoperability between dynamically loaded modules.
    pub atom_defs: Vec<Atom<Sym>>,
    pub type_defs: Vec<TypeDesc>,

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
    pub (crate) item: usize,
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
    Concrete(Adapted<Reference>),
    Generic(u32),
}

/// A type
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeDesc {
    /// References here are to atoms
    AtomGroup(Vec<Adapted<Reference>>),
    /// Though curried, these functions do not map cleanly to most language's curried functions.
    /// Wrappers may be necessary to make functions the desired type.
    Func {
        input: ArgType,
        /// Must either be another func or BottomThunk
        output: ArgType,
    },
    /// References here are to TypeDescs
    Tuple(Vec<Reference>),
    /// A computation that never returns
    BottomThunk,
    Int8,
    Int16,
    Int32,
    Int64,
    //Float16??
    Float32,
    Float64,
    /// Contents of typedef is a set of pairs of symbols and function types.
    Library(Vec<(Reference, Adapted<Reference>)>),
    Array(Reference),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adapted<T> {
    pub params: Vec<ArgType>,
    pub item: T,
}
