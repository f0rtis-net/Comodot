use module_builder::{build_module, BuildingModule};

fn main() {
    build_module(&BuildingModule {
        name: "test_proj",
        path: "/Users/ivankaravaev/Documents/Comodot/compiler/test_proj",
        files: vec![
            "/Users/ivankaravaev/Documents/Comodot/compiler/test_proj/main.cd",
            "/Users/ivankaravaev/Documents/Comodot/compiler/test_proj/another_in_module.cd"
            ]
    });
}