use clap::Arg;
use module_builder::{build_module, BuildingModule};

fn main() {
    build_module(&BuildingModule {
        name: "lol",
        path: "/Users/ivankaravaev/Documents/Comodot/compiler/test_proj/main.cd",
        files: Vec::new()
    });
}