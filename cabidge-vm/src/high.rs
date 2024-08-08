use std::{collections::HashMap, path::PathBuf};

use cabidge_code::{func::{Arg, Function as Func, Synced, Terminal, Value}, r#gen::{IndexMap, IndexSet, IndexVec}, module::{Reference, TypeDesc}};

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

pub struct AtomDesc {
    pub source: Reference<PathBuf>,
    pub name: Reference<String>
}

pub struct Atom {
    pub desc: AtomDesc,
    pub num_members: usize,
}

pub struct ModMerge {
    symbols: IndexSet<String>,

    modules: IndexSet<PathBuf>,
    atoms: IndexMap<AtomDesc, usize>,

    types: IndexSet<TypeDesc>,

    func_defs: IndexVec<Option<Func>>,
    func_names: HashMap<(Reference<PathBuf>, Reference<String>), Reference<Option<Func>>>
}