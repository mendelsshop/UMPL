use inkwell::{module::Module, builder::Builder, context::Context};

use super::sicp::Instruction;

pub struct CodeGen<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    labels: (),
}

impl <'a, 'ctx> CodeGen  <'a, 'ctx> {
    pub fn compile(instructions: Vec<Instruction>) {
        instructions.into_iter().filter_map(|x| Some(x)); // if its label then add block and push to labels and rreturn none otherwuise some

    }

    pub fn compile_instructions(instruction: Instruction) {
        match instruction {
            Instruction::Assign(_, _) => todo!(),
            Instruction::Test(_) => todo!(),
            Instruction::Branch(_) => todo!(),
            Instruction::Goto(_) => todo!(),
            Instruction::Save(_) => todo!(),
            Instruction::Restore(_) => todo!(),
            Instruction::Perform(_) => todo!(),
            Instruction::Label(_) => unreachable!(),
            Instruction::Nop => todo!(),
        }
    }

}