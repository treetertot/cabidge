use std::{
    collections::BTreeSet, fs::File, hash::Hash, marker::PhantomData, path::{Path, PathBuf}
};

use serde::{Deserialize, Serialize};

use crate::func::Function;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub symbols: Vec<String>,

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
    pub func_uses: Vec<Use<Function>>,
}
impl Module {
    pub fn write_to_file<P>(&self, path: P) -> bincode::Result<()>
    where
        P: AsRef<Path>,
    {
        let file = File::create(path)
            .map_err(bincode::ErrorKind::Io)
            .map_err(Box::new)?;
        bincode::serialize_into(file, self)
    }
    pub fn read_from_file<P>(&self, path: P) -> bincode::Result<()>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)
            .map_err(bincode::ErrorKind::Io)
            .map_err(Box::new)?;
        bincode::deserialize_from(file)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference<T> {
    item: usize,
    _tag: PhantomData<T>,
}
impl<T> Reference<T> {
    pub(crate) fn new(item: usize) -> Self {
        Reference {
            item,
            _tag: PhantomData,
        }
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
impl<T> PartialEq for Reference<T> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}
impl<T> Eq for Reference<T> {}
impl<T> PartialOrd for Reference<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.item.partial_cmp(&other.item)
    }
}
impl<T> Ord for Reference<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.item.cmp(&other.item)
    }
}
impl <T> Hash for Reference<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.item)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Export {
    pub rf: Reference<Use<Function>>,
    pub name: Reference<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Import<T> {
    pub name: Reference<String>,
    pub source: Reference<PathBuf>,
    _tag: PhantomData<T>,
}
impl<T> Import<T> {
    pub fn new(name: Reference<String>, source: Reference<PathBuf>) -> Self {
        Import {
            name,
            source,
            _tag: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Use<T> {
    Internal(Reference<T>),
    External(Reference<Import<T>>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Atom {
    /// All atoms are exported
    pub name: Reference<String>,
    pub num_members: usize,
}

/// Type of a function or type argument/param
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArgType {
    Concrete(Adapted<Reference<TypeDesc>>),
    Generic(u32),
}

/// A type
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeDesc {
    /// References here are to atoms
    AtomGroup(BTreeSet<Adapted<Reference<Atom>>>),
    /// Though curried, these functions do not map cleanly to most language's curried functions.
    /// Wrappers may be necessary to make functions the desired type.
    Func(Vec<ArgType>),
    /// References here are to TypeDescs
    Tuple(Vec<Reference<TypeDesc>>),
    /// A computation that never returns
    BottomThunk,
    //Int8,
    //Int16,
    Int32,
    //Int64,
    //Float16??
    //Float32,
    //Float64,
    // /// Contents of typedef is a set of pairs of symbols and function types.
    //Library(Vec<(Reference<String>, Adapted<Reference<TypeDesc>>)>),
    //Array(Reference), // later
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Adapted<T> {
    pub params: Vec<ArgType>,
    pub item: T,
}
