use std::collections::{hash_map::Entry, HashMap};

use hir::HirId;

pub mod ty;

pub enum BuildType {
    Executable, 
    ModulePack
}

pub struct GlobalCtx {
    module_name: String,
    module_ty_info: HirModuleTypeTable,
    module_exports: Vec<HirId>,
    arch: String,
    build_type: BuildType
}

impl GlobalCtx {
    pub fn new(module_name: String, arch: String, build_type: BuildType) -> GlobalCtx {
        GlobalCtx {
            module_name,
            module_ty_info: HirModuleTypeTable::new(),
            module_exports: Vec::new(),
            arch,
            build_type
        }
    }

    pub fn get_build_ty(&self) -> &BuildType {
        &self.build_type
    }

    pub fn get_arch(&self) -> String {
        self.arch.clone()
    }

    pub fn get_module_name(&self) -> String {
        self.module_name.clone()
    }

    pub fn get_module_ty_info(&mut self) -> &mut HirModuleTypeTable {
        &mut self.module_ty_info
    }

    pub fn get_module_exports(&mut self) -> &mut Vec<HirId> {
        &mut self.module_exports
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
