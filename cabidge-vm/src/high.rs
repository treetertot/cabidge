use std::{collections::{HashMap, HashSet}, path::PathBuf};

use cabidge_code::{func::{Arg, Function as Func, Synced, Terminal, Value}, r#gen::{IndexMap, IndexSet, IndexVec, RefSet}, module::{Atom as ModAtom, Module, Reference, TypeDesc}};

pub enum Op {
    Terminate(Value),
    // Any I/O operators will also need a continuation parameter
    WriteStd(Value),
    //LoadPath(Value),
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),

    LoadFunc(Reference<Function>),
    LoadAtom(Reference<Atom>),
    Apply {
        f: Value,
        args: Vec<Value>
    }
}

pub struct Function {
    args: Vec<Arg>,
    assgns: Vec<Assignment>,
    terminal: Terminal<Atom>
}

pub struct Assignment {
    val: Value,
    t_annotation: Synced<TypeDesc>,
    op: Op
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AtomDesc {
    pub source: Reference<PathBuf>,
    pub name: Reference<String>
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub desc: AtomDesc,
    pub num_members: usize,
}

#[derive(Debug, Default)]
pub struct ModMerge {
    symbols: IndexSet<String>,

    absorbed_modules: RefSet<Reference<PathBuf>>,

    modules: IndexSet<PathBuf>,
    atoms: IndexMap<AtomDesc, usize>,

    types: IndexSet<TypeDesc>,

    func_defs: IndexVec<Option<Func>>,
    func_names: HashMap<(Reference<PathBuf>, Reference<String>), Reference<Option<Func>>>
}
impl ModMerge {
    pub fn absorb(&mut self, path: PathBuf, module: Module) -> () {
        
    } 
}