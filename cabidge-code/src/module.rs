use std::{fs::File, marker::PhantomData, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

use crate::func::Function;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module<Sym> {
    pub symbols: Vec<Sym>,

    pub imported_modules: Vec<PathBuf>,
    /// Atoms are automatically exported
    pub atom_defs: Vec<Atom>,
    pub atom_imports: Vec<Import<Atom>>,
    pub atom_uses: Vec<Use<Atom>>,
    /// Types are anonymous and therefore not namespaced or importable.
    /// This is for easy interoperability between dynamically loaded modules.
    pub type_defs: Vec<TypeDesc>,

    pub func_defs: Vec<Function>,
    pub func_imports: Vec<Import<Function>>,
    pub func_exports: Vec<Export>,
    pub func_uses: Vec<Use<Function>>
}
impl<Sym> Module<Sym> {
    pub fn write_to_file<P>(&self, path: P) -> bincode::Result<()>
    where
        P: AsRef<Path>,
        Sym: Serialize
    {
        let file = File::create(path).map_err(bincode::ErrorKind::Io).map_err(Box::new)?;
        bincode::serialize_into(file, self)
    }
    pub fn read_from_file<P>(&self, path: P) -> bincode::Result<()>
    where
        P: AsRef<Path>,
        for<'de> Sym: Deserialize<'de>
    {
        let file = File::open(path).map_err(bincode::ErrorKind::Io).map_err(Box::new)?;
        bincode::deserialize_from(file)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolTag;

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
impl<T> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Self::new(self.item)
    }
}
impl<T> Copy for Reference<T> {}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Export {
    pub rf: Reference<Use<Function>>,
    pub name: Reference<SymbolTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Import<T> {
    pub name: Reference<SymbolTag>,
    pub source: Reference<PathBuf>,
    _tag: PhantomData<T>,
}
impl<T> Import<T> {
    pub fn new(name: Reference<SymbolTag>, source: Reference<PathBuf>) -> Self {
        Import { name, source, _tag: PhantomData }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Use<T> {
    Internal(Reference<T>),
    External(Reference<Import<T>>)
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
    // /// Contents of typedef is a set of pairs of symbols and function types.
    //Library(Vec<(Reference<SymbolTag>, Adapted<Reference<TypeDesc>>)>),
    //Array(Reference), // later
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adapted<T> {
    pub params: Vec<ArgType>,
    pub item: T,
}
