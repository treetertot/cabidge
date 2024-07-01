//! Codegen tools
//! 
//! The language here is kinda lazy I think??
//! You can check the func module for the behavior of [Function]s.

use std::{collections::HashMap, hash::Hash, path::PathBuf, str::FromStr};

use crate::{func::Function, module::{Atom, Export, Import, Module, Reference, TypeDesc}};

#[derive(Debug, Clone, Default)]
pub struct IndexSet<T> {
    dedup: HashMap<T, Reference>,
    items: Vec<T>,
}
impl<T: Clone + Hash + Eq> IndexSet<T> {
    pub fn new() -> IndexSet<T> {
        IndexSet { dedup: HashMap::new(), items: Vec::new() }
    }
    pub fn insert(&mut self, item: T) -> Reference {
        if let Some(idx) = self.dedup.get(&item) {
            return *idx;
        }
        let idx = Reference { item: self.items.len() };
        self.dedup.insert(item.clone(), idx);
        self.items.push(item);
        idx
    }
    pub fn finish(self) -> Vec<T> {
        self.items
    }
}

#[derive(Debug, Clone)]
pub struct ModuleBuilder<Sym> {
    module_imports: Vec<PathBuf>,

    atom_imports: Vec<Import<Sym>>,
    atom_names: IndexSet<Sym>,
    atom_args: Vec<usize>,

    type_defs: IndexSet<TypeDesc>,

    value_imports: Vec<Import<Sym>>,
    value_defs: Vec<Function>,
    value_exports: Vec<Export<Sym>>,
}
impl<Sym: Clone + Hash + Eq> ModuleBuilder<Sym> {
    pub fn new() -> ModuleBuilder<Sym> {
        ModuleBuilder {
            module_imports: Vec::new(),

            atom_imports: Vec::new(),
            atom_names: IndexSet::new(),
            atom_args: Vec::new(),

            type_defs: IndexSet::new(),

            value_imports: Vec::new(),
            value_defs: Vec::new(),
            value_exports: Vec::new(),
        }
    }
    pub fn add_atom(&mut self, name: Sym, num_members: usize) -> Reference {
        let output = self.atom_names.insert(name);
        let is_append = output.item == self.atom_args.len();
        if is_append {
            self.atom_args.push(num_members);
        }
        output
    }
    pub fn export_value(&mut self, name: Sym, rf: Reference) {
        self.value_exports.push(Export { rf, name });
    }
    pub fn add_function(&mut self, func: Function) -> Reference {
        let item = self.value_defs.len();
        self.value_defs.push(func);
        Reference { item }
    }
    pub fn register_type(&mut self, type_desc: TypeDesc) -> Reference {
        self.type_defs.insert(type_desc)
    }
    pub fn finish(self) -> Module<Sym> {
        let Self { mut module_imports, atom_imports, atom_names, atom_args, type_defs, value_imports, value_defs, value_exports } = self;
        module_imports.push(PathBuf::from_str("src").unwrap());
        let atom_defs = atom_names.finish().into_iter().zip(atom_args).map(|(name, num_members)| Atom{ name, num_members }).collect();
        Module {
            module_imports,

            atom_imports,
            atom_defs,

            type_defs: type_defs.finish(),

            value_imports,
            value_defs,
            value_exports,
        }
    }
}
