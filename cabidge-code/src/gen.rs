//! Codegen tools
//! 
//! The language here is kinda lazy I think??
//! You can check the func module for the behavior of [Function]s.

use std::{collections::HashMap, hash::Hash, path::PathBuf};

use crate::{func::Function, module::{Atom, Export, Import, Module, Reference, TypeDesc, Use}};

#[derive(Debug, Clone)]
pub struct IndexMap<T, I> {
    dedup: HashMap<T, (Reference<T>, I)>,
    items: Vec<(T, I)>,
}
impl<T: Clone + Hash + Eq, I: Clone + Eq> IndexMap<T, I> {
    pub fn new() -> IndexMap<T, I> {
        IndexMap { dedup: HashMap::new(), items: Vec::new() }
    }
    pub fn insert(&mut self, item: T, data: I) -> Result<Reference<T>, CodeGenErr> {
        if let Some(idx) = self.dedup.get(&item) {
            if idx.1 != data {
                return Err(CodeGenErr::NameCollision);
            }
            return Ok(idx.0.clone());
        }
        let idx = Reference::new(self.items.len());
        self.dedup.insert(item.clone(), (idx.clone(), data.clone()));
        self.items.push((item, data));
        Ok(idx)
    }
    pub fn finish(self) -> Vec<(T, I)> {
        self.items
    }
}
impl<T: Clone + Hash + Eq, I: Clone + Eq> Default for IndexMap<T, I> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct IndexSet<T> (IndexMap<T, ()>);
impl<T: Clone + Hash + Eq> IndexSet<T> {
    pub fn new() -> IndexSet<T> {
        Self (
            Default::default()
        )
    }
    pub fn insert(&mut self, item: T) -> Reference<T> {
        self.0.insert(item, ()).unwrap()
    }
    pub fn finish(self) -> Vec<T> {
        self.0.finish().into_iter().map(|(t, ())| t).collect()
    }
}
impl<T: Clone + Hash + Eq> Default for IndexSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct IndexVec<T> (Vec<T>);
impl<T> IndexVec<T> {
    pub fn new() -> Self {
        IndexVec(Vec::new())
    }
    pub fn insert(&mut self, item: T) -> Reference<T> {
        let r = Reference::new(self.0.len());
        self.0.push(item);
        r
    }
    pub fn finish(self) -> Vec<T> {
        self.0
    }
}
impl<T> Default for IndexVec<T> {
    fn default() -> Self {
        IndexVec::new()
    }
}

#[derive(Debug, Clone)]
pub struct ModuleBuilder {
    symbols: IndexSet<String>,

    imported_modules: IndexSet<PathBuf>,

    atom_defs: IndexMap<Reference<String>, usize>,
    atom_imports: IndexSet<Import<Atom>>,
    atom_uses: IndexSet<Use<Atom>>,

    type_defs: IndexSet<TypeDesc>,

    func_defs: IndexVec<Function>,
    func_imports: IndexSet<Import<Function>>,
    func_exports: IndexVec<Export>,
    func_uses: IndexSet<Use<Function>>,
}
impl ModuleBuilder {
    pub fn new() -> Self {
        ModuleBuilder {
            symbols: Default::default(),

            imported_modules: Default::default(),

            atom_defs: Default::default(),
            atom_imports: Default::default(),
            atom_uses: Default::default(),

            type_defs: Default::default(),

            func_defs: Default::default(),
            func_imports: Default::default(),
            func_exports: Default::default(),
            func_uses: Default::default(),
        }
    }
    pub fn import_module(&mut self, path: PathBuf) -> Reference<PathBuf> {
        self.imported_modules.insert(path)
    }
    pub fn create_atom(&mut self, name: String, num_members: usize) -> Result<Reference<Use<Atom>>, CodeGenErr> {
        let s = self.symbols.insert(name).retag();
        let r = self.atom_defs.insert(s, num_members)?.retag();
        Ok(self.atom_uses.insert(Use::Internal(r)))
    }
    pub fn import_atom(&mut self, name: String, source: Reference<PathBuf>) -> Reference<Use<Atom>> {
        let s = self.symbols.insert(name).retag();
        let a = self.atom_imports.insert(Import::new(s, source));
        self.atom_uses.insert(Use::External(a))
    }
    pub fn register_type(&mut self, type_desc: TypeDesc) -> Reference<TypeDesc> {
        self.type_defs.insert(type_desc)
    }
    pub fn create_function(&mut self, func: Function) -> Reference<Use<Function>> {
        let f = self.func_defs.insert(func);
        self.func_uses.insert(Use::Internal(f))
    }
    pub fn export_function(&mut self, name: String, rf: Reference<Use<Function>>) {
        let s = self.symbols.insert(name).retag();
        self.func_exports.insert(Export { rf, name: s });
    }
    pub fn import_function(&mut self, name: String, source: Reference<PathBuf>) -> Reference<Use<Function>> {
        let s = self.symbols.insert(name).retag();
        let import = self.func_imports.insert(Import::new(s, source));
        self.func_uses.insert(Use::External(import))
    }
    pub fn finish(self) -> Module {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CodeGenErr {
    NameCollision,
}