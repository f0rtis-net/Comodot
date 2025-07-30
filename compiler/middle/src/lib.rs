use std::{cell::RefCell, collections::{hash_map::Entry, HashMap}};

use hir::{HirFile, HirId};

pub mod ty;

pub enum BuildType {
    Executable, 
    ModulePack
}

pub struct GlobalCtx<'a> {
    pub module_name: String,
    pub module_ty_info: RefCell<HirModuleTypeTable>,
    pub module_symbols: RefCell<NamePairs>,
    pub module_exports: Vec<HirId>,
    pub module_files: Vec<HirFile<'a>>,
    pub arch: String,
    pub build_type: BuildType
}

impl<'a> GlobalCtx<'a> {
    pub fn new(module_name: String, arch: String, build_type: BuildType) -> Self {
        Self {
            module_name,
            module_ty_info: RefCell::new(HirModuleTypeTable::new()),
            module_exports: Vec::new(),
            arch,
            module_symbols: RefCell::new(NamePairs::new()),
            module_files: Vec::new(),
            build_type
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolInfo {
    pub id: HirId,
    pub is_external_name: bool
}


pub struct NamePairs {
    pairs: HashMap<HirId, SymbolInfo>,
}

impl NamePairs {
    pub fn new() -> NamePairs {
        NamePairs {
            pairs: HashMap::new()
        }
    }

    pub fn add_pair(&mut self, hir_id: HirId, symbol_info: SymbolInfo) {
        self.pairs.insert(hir_id, symbol_info);
    }

    pub fn get_pair(&self, hir_id: &HirId) -> Option<&SymbolInfo> {
        self.pairs.get(hir_id)
    }
}

#[derive(Clone)]
pub struct TypeInfo {
    pub ty: ty::LangType,
    pub inferred: bool,
}

pub struct HirModuleTypeTable {
    types: HashMap<HirId, TypeInfo>
}

impl HirModuleTypeTable {
    pub fn new() -> HirModuleTypeTable {
        HirModuleTypeTable {
            types: HashMap::new()
        }
    }

    pub fn insert_type(&mut self, hir_id: HirId, ty: TypeInfo) {
        match self.types.entry(hir_id) {
            Entry::Occupied(mut entry) => {
                let existing_info = entry.get_mut();
            
                *existing_info = ty;    
            }
            Entry::Vacant(entry) => {
                entry.insert(ty);
            }
        }
    }

    pub fn get_type(&self, hir_id: &HirId) -> Option<&TypeInfo> {
        self.types.get(hir_id)
    }

    pub fn dump(&mut self) {
        for (hir_id, ty) in &self.types {
            println!("{:?}: {:?}", hir_id, ty.ty);
        }
    }
}
