use std::path::PathBuf;

use inkwell::{
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};

use super::Compiler;

// exporting functions
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn export_ir(&self, path: impl Into<PathBuf>) {
        let mut path: PathBuf = path.into();
        path.set_extension("ll");
        self.module.print_to_file(&path).expect("couldn't export");
    }

    pub fn export_bc(&self, path: impl Into<PathBuf>) {
        let mut path: PathBuf = path.into();
        path.set_extension("bc");
        self.module.write_bitcode_to_path(&path);
    }

    // TODO: split into asm and object functions
    pub fn export_object_and_asm(&self, path: impl Into<PathBuf>) {
        let mut asm_path: PathBuf = path.into();
        let mut o_path: PathBuf = asm_path.clone();
        o_path.set_extension("o");
        asm_path.set_extension("as");

        let config = InitializationConfig {
            asm_parser: true,
            asm_printer: true,
            base: true,
            disassembler: true,
            info: true,
            machine_code: true,
        };

        Target::initialize_native(&config).unwrap();
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).unwrap();
        let tm = target
            .create_target_machine(
                &TargetMachine::get_default_triple(),
                &TargetMachine::get_host_cpu_name().to_string(),
                &TargetMachine::get_host_cpu_features().to_string(),
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();
        tm.set_asm_verbosity(true);
        tm.add_analysis_passes(self.fpm);

        tm.write_to_file(self.module, inkwell::targets::FileType::Object, &o_path)
            .expect(" writing to file ");

        tm.write_to_file(self.module, inkwell::targets::FileType::Assembly, &asm_path)
            .expect(" writing to file ");
    }
}
