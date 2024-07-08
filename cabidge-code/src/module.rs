use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::func::Function;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module<Sym> {
    pub symbols: Vec<Sym>,
    /// Atoms and types are both universal and tehrefore not namespaced.
    /// This is for easy interoperability between dynamically loaded modules.
    pub atom_defs: Vec<Atom>,
    pub type_defs: Vec<TypeDesc>,

    pub value_defs: Vec<Function>,
    pub value_exports: Vec<Export>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolTag;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reference<T> {
    item: usize,
    _tag: PhantomData<T>
}
impl<T> Reference<T> {
    pub(crate) fn new(item: usize) -> Self {
        Reference { item, _tag: PhantomData }
    }
    pub fn get_inner(&self) -> usize {
        self.item
    }
    pub fn retag<O>(self) -> Reference<O> {
        Reference::new(self.get_inner())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub rf: Reference<Function>,
    pub name: Reference<SymbolTag>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Atom {
    /// All atoms are exported
    pub name: Reference<SymbolTag>,
    pub num_members: usize,
}

/// Type of a function or type argument/param
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgType {
    Concrete(Adapted<Reference<TypeDesc>>),
    Generic(u32),
}

/// A type
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeDesc {
    /// References here are to atoms
    AtomGroup(Vec<Adapted<Reference<Atom>>>),
    /// Though curried, these functions do not map cleanly to most language's curried functions.
    /// Wrappers may be necessary to make functions the desired type.
    Func {
        input: ArgType,
        /// Must either be another func or BottomThunk
        output: ArgType,
    },
    /// References here are to TypeDescs
    Tuple(Vec<Reference<TypeDesc>>),
    /// A computation that never returns
    BottomThunk,
    //Int8,
    //Int16,
    //Int32,
    //Int64,
    //Float16??
    //Float32,
    //Float64,
    /// Contents of typedef is a set of pairs of symbols and function types.
    Library(Vec<(Reference<SymbolTag>, Adapted<Reference<TypeDesc>>)>),
    //Array(Reference), // later
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adapted<T> {
    pub params: Vec<ArgType>,
    pub item: T,
}
