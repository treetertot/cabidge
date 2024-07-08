//! Codegen tools
//! 
//! The language here is kinda lazy I think??
//! You can check the func module for the behavior of [Function]s.

use std::{collections::HashMap, hash::Hash};

use crate::{func::Function, module::{Atom, Export, Module, Reference, TypeDesc}};

#[derive(Debug, Clone, Default)]
pub struct IndexSet<T> {
    dedup: HashMap<T, Reference<T>>,
    items: Vec<T>,
}
impl<T: Clone + Hash + Eq> IndexSet<T> {
    pub fn new() -> IndexSet<T> {
        IndexSet { dedup: HashMap::new(), items: Vec::new() }
    }
    pub fn insert(&mut self, item: T) -> Reference<T> {
        if let Some(idx) = self.dedup.get(&item) {
            return idx.clone();
        }
        let idx = Reference::new(self.items.len());
        self.dedup.insert(item.clone(), idx.clone());
        self.items.push(item);
        idx
    }
    pub fn finish(self) -> Vec<T> {
        self.items
    }
}

#[derive(Debug, Clone)]
pub struct ModuleBuilder<Sym> {
    symbols: IndexSet<Sym>,

    atoms: IndexSet<Atom>,

    type_defs: IndexSet<TypeDesc>,

    value_defs: Vec<Function>,
    value_exports: Vec<Export>,
}
impl<Sym: Clone + Hash + Eq> ModuleBuilder<Sym> {
    pub fn new() -> ModuleBuilder<Sym> {
        ModuleBuilder {
            symbols: IndexSet::new(),

            atoms: IndexSet::new(),

            type_defs: IndexSet::new(),

            value_defs: Vec::new(),
            value_exports: Vec::new(),
        }
    }
    pub fn add_atom(&mut self, name: Sym, num_members: usize) -> Reference<Atom> {
        let name = self.symbols.insert(name).retag();
        self.atoms.insert(Atom { name, num_members })
    }
    pub fn export_value(&mut self, name: Sym, rf: Reference<Function>) {
        let name = self.symbols.insert(name).retag();
        self.value_exports.push(Export { rf, name });
    }
    pub fn add_function(&mut self, func: Function) -> Reference<Function> {
        let item = self.value_defs.len();
        self.value_defs.push(func);
        Reference::new(item)
    }
    pub fn register_type(&mut self, type_desc: TypeDesc) -> Reference<TypeDesc> {
        self.type_defs.insert(type_desc)
    }
    pub fn finish(self) -> Module<Sym> {
        let Self { symbols, atoms, type_defs, value_defs, value_exports } = self;
        Module {
            symbols: symbols.finish(),
            atom_defs: atoms.finish(),

            type_defs: type_defs.finish(),

            value_defs,
            value_exports,
        }
    }
}
