use std::{env::temp_dir, fs::File, io::Write};

use move_binary_format::CompiledModule;
use move_compiler::compiled_unit::AnnotatedCompiledUnit;
use move_compiler::Compiler;

pub fn compile_code(code: String) -> anyhow::Result<Vec<AnnotatedCompiledUnit>> {
    let dir = temp_dir();
    let file_path = dir.as_path().join("module.move");

    {
        let mut file = File::create(&file_path).unwrap();
        write!(&mut file, "{}", code).unwrap();
    }

    let targets = vec![file_path.to_str().unwrap().to_string()];
    let compiler =
        Compiler::from_files(targets, vec![], move_stdlib::move_stdlib_named_addresses());

    let (_, units) = compiler.build_and_report()?;
    Ok(units)
}

pub fn get_module(unit: AnnotatedCompiledUnit) -> CompiledModule {
    match unit {
        AnnotatedCompiledUnit::Module(module) => module.named_module.module,
        _ => panic!(""),
    }
}
