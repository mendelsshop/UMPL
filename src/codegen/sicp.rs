use std::{
    collections::HashSet,
    fmt::{self},
};

use itertools::Itertools;

use crate::ast::UMPL2Expr;

type Label = String;

// https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd45a
macro_rules! hashset {
    ($($element:expr),*) => {{
        // check that count is const
        const C: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut vs = HashSet::with_capacity(C);
        $(vs.insert ($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::hashset![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Linkage {
    Return,
    Next,
    Label(Label),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Register {
    Env,
    Argl,
    Val,
    Proc,
    Continue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    LookupVariableValue,
    CompiledProcedureEnv,
    CompiledProcedureEntry,
    DefineVariable,
    ApplyPrimitiveProcedure,
    ExtendEnvoirnment,
    Cons,
    SetVariableValue,
    False,
    RandomBool,
    MakeCompiledProcedure,
    PrimitiveProcedure,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Env => write!(f, "env"),
            Self::Argl => write!(f, "argl"),
            Self::Val => write!(f, "val"),
            Self::Proc => write!(f, "proc"),
            Self::Continue => write!(f, "continue"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Empty,
    String(String),
    Symbol(String),
    Number(f64),
    Boolean(bool),
    List(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Const(Const),
    Label(Label),
    Register(Register),
    Op(Perform),
}
impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List(e1, e2) => {
                let mut e2 = *e2.clone();
                write!(f, "{e1}")?;
                while let Expr::Const(Self::List(ne1, ne2)) = e2.clone() {
                    write!(f, " {ne1}",)?;
                    e2 = *ne2;
                }
                if e2 == Expr::Const(Self::Empty) {
                    write!(f, ")")
                } else {
                    write!(f, " . {e2}")
                }
            }
            Self::String(s) => write!(f, "{s}"),
            Self::Symbol(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Empty => write!(f, "()"),
            Self::Boolean(b) => write!(f, "{b}"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(s) => write!(f, "(const {s})",),
            Self::Label(l) => write!(f, "(label {l})",),
            Self::Register(r) => write!(f, "(reg {r})",),
            Self::Op(p) => write!(f, "{p}",),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Goto {
    Label(Label),
    Register(Register),
}

impl fmt::Display for Goto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label(l) => write!(f, "(label {l})"),
            Self::Register(r) => write!(f, "(reg {r})"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Perform {
    op: Operation,
    args: Vec<Expr>,
}

impl Perform {
    pub fn op(&self) -> &Operation {
        &self.op
    }

    pub fn args(&self) -> &[Expr] {
        self.args.as_ref()
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decamel = |str: String| {
            str.split_inclusive(|x: char| x.is_uppercase())
                .map(str::to_lowercase)
                .join("-")
        };
        let kebabified = decamel(format!("{self:?}"));
        match self {
            Self::False | Self::PrimitiveProcedure => write!(f, "{kebabified}?"),
            _ => write!(f, "{kebabified}"),
        }
    }
}
impl fmt::Display for Perform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(op {}) {}",
            self.op,
            self.args
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Assign(Register, Expr),
    Test(Perform),
    Branch(Label),
    Goto(Goto),
    Save(Register),
    Restore(Register),
    Perform(Perform),
    Label(Label),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(r, e) => write!(f, " (assign {r} {e})"),
            Self::Test(p) => write!(f, " (test {p})",),
            Self::Branch(l) => write!(f, " (branch (label {l}))",),
            Self::Goto(g) => write!(f, " (goto {g})",),
            Self::Save(r) => write!(f, " (save {r})",),
            Self::Restore(r) => write!(f, " (restore {r})",),
            Self::Perform(r) => write!(f, " (perform {r})",),
            Self::Label(l) => write!(f, "{l}",),
        }
    }
}

pub struct InstructionSequnce {
    needs: HashSet<Register>,
    modifiers: HashSet<Register>,
    instructions: Vec<Instruction>,
}

impl InstructionSequnce {
    fn new(
        needs: HashSet<Register>,
        modifiers: HashSet<Register>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self {
            needs,
            modifiers,
            instructions,
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        self.instructions.as_ref()
    }
}

pub fn compile(exp: UMPL2Expr, target: Register, linkage: Linkage) -> InstructionSequnce {
    match exp {
        UMPL2Expr::Ident(i) => compile_variable(i.to_string(), target, linkage),
        UMPL2Expr::Application(mut a) => match a.first() {
            Some(UMPL2Expr::Ident(i)) => match i.to_string().as_str() {
                "quote" => compile_quoted(a, target, linkage),
                "set!" => compile_assignment(a, target, linkage),
                "define" => compile_defeninition(a, target, linkage),
                "if" => compile_if(a, target, linkage),
                "lambda" => compile_lambda(a, target, linkage),
                "begin" => compile_seq(
                    {
                        a.remove(0);
                        a
                    },
                    target,
                    linkage,
                ),
                "cond" => todo!(),
                "let" => todo!(),
                _ => compile_application(a, target, linkage),
            },
            Some(_) => compile_application(a, target, linkage),
            None => todo!(),
        },
        UMPL2Expr::FnParam(i) => compile_variable(i.to_string(), target, linkage),
        exp => compile_self_evaluating(exp.into(), target, linkage),
    }
}

fn empty_instruction_seqeunce() -> InstructionSequnce {
    InstructionSequnce::new(hashset![], hashset![], vec![])
}

fn compile_linkage(linkage: Linkage) -> InstructionSequnce {
    match linkage {
        Linkage::Return => InstructionSequnce::new(
            hashset![],
            hashset![],
            vec![Instruction::Goto(Goto::Register(Register::Continue))],
        ),
        Linkage::Next => empty_instruction_seqeunce(),
        Linkage::Label(label) => InstructionSequnce::new(
            hashset![],
            hashset![],
            vec![Instruction::Goto(Goto::Label(label))],
        ),
    }
}

fn parallel_instruction_sequnce(
    instruction_seq1: InstructionSequnce,
    instruction_seq2: InstructionSequnce,
) -> InstructionSequnce {
    let mut needs = instruction_seq1.needs;
    needs.extend(instruction_seq2.needs);
    let mut modifiers = instruction_seq1.modifiers;
    modifiers.extend(instruction_seq2.modifiers);
    let mut instructions = instruction_seq1.instructions;
    instructions.extend(instruction_seq2.instructions);
    InstructionSequnce::new(needs, modifiers, instructions)
}

fn append_instruction_sequnce(
    mut seq1: InstructionSequnce,
    seq2: InstructionSequnce,
) -> InstructionSequnce {
    seq1.instructions.extend(seq2.instructions);
    seq1.needs.extend(seq2.needs);
    seq1.modifiers.extend(seq2.modifiers);
    seq1
}

fn preserving(
    register: HashSet<Register>,
    instruction_seq1: InstructionSequnce,
    instruction_seq2: InstructionSequnce,
) -> InstructionSequnce {
    if register.is_empty() {
        append_instruction_sequnce(instruction_seq1, instruction_seq2)
    } else {
        let used_registers = instruction_seq1
            .modifiers
            .intersection(&instruction_seq2.needs);
        let used_registers = used_registers.filter(|r| register.contains(r));

        let mut instructions = vec![];
        instructions.extend(used_registers.clone().map(|r| Instruction::Save(*r)));
        instructions.extend(instruction_seq1.instructions);
        instructions.extend(used_registers.map(|r| Instruction::Restore(*r)));
        instructions.extend(instruction_seq2.instructions);
        let mut needs = instruction_seq1.needs;
        needs.extend(instruction_seq2.needs);
        let mut modifiers = instruction_seq1.modifiers;
        modifiers.extend(instruction_seq2.modifiers);
        InstructionSequnce::new(
            needs.into_iter().collect(),
            modifiers.into_iter().collect(),
            instructions,
        )
    }
}

fn end_with_linkage(
    linkage: Linkage,
    instruction_sequnce: InstructionSequnce,
) -> InstructionSequnce {
    preserving(
        hashset![Register::Continue],
        instruction_sequnce,
        compile_linkage(linkage),
    )
}

fn compile_self_evaluating(exp: Expr, target: Register, linkage: Linkage) -> InstructionSequnce {
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![],
            hashset![target],
            vec![Instruction::Assign(target, exp)],
        ),
    )
}

fn compile_quoted(
    mut exp: Vec<UMPL2Expr>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    let Some(quoted) = exp.pop() else { panic!() };
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![],
            hashset![target],
            vec![Instruction::Assign(target, quoted.into())],
        ),
    )
}

static mut LABEL_COUNTER: usize = 0;

fn label_counter() -> usize {
    unsafe {
        LABEL_COUNTER += 1;
        LABEL_COUNTER
    }
}

fn make_label_name(label: String) -> Label {
    format!("{}{}", label, label_counter())
}

fn compile_variable(exp: String, target: Register, linkage: Linkage) -> InstructionSequnce {
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![Register::Env],
            hashset![target],
            vec![Instruction::Assign(
                target,
                Expr::Op(Perform {
                    op: Operation::LookupVariableValue,
                    args: vec![
                        Expr::Const(Const::Symbol(exp)),
                        Expr::Register(Register::Env),
                    ],
                }),
            )],
        ),
    )
}

fn compile_assignment(
    exp: Vec<UMPL2Expr>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    let var = match exp.get(1) {
        Some(UMPL2Expr::Ident(i)) => i.to_string(),
        _ => panic!(),
    };
    let get_value_code = exp.get(2).map_or_else(
        || panic!(),
        |v| compile(v.clone(), Register::Val, Linkage::Next),
    );
    end_with_linkage(
        linkage,
        preserving(
            hashset![Register::Env],
            get_value_code,
            InstructionSequnce::new(
                hashset![Register::Env, Register::Val],
                hashset![target],
                vec![
                    Instruction::Assign(
                        target,
                        Expr::Op(Perform {
                            op: Operation::SetVariableValue,
                            args: vec![
                                Expr::Const(Const::Symbol(var)),
                                Expr::Register(Register::Val),
                                Expr::Register(Register::Env),
                            ],
                        }),
                    ),
                    Instruction::Assign(target, Expr::Const(Const::Symbol("ok".to_string()))),
                ],
            ),
        ),
    )
}

fn compile_defeninition(
    exp: Vec<UMPL2Expr>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    let var = match exp.get(1) {
        Some(UMPL2Expr::Ident(i)) => i.to_string(),
        // Some(UMPL2Expr::Applicati
        _ => panic!(),
    };
    let get_value_code = exp.get(2).map_or_else(
        || panic!(),
        |v| compile(v.clone(), Register::Val, Linkage::Next),
    );
    end_with_linkage(
        linkage,
        preserving(
            hashset![Register::Env],
            get_value_code,
            InstructionSequnce::new(
                hashset![Register::Env, Register::Val],
                hashset![target],
                vec![
                    Instruction::Assign(
                        target,
                        Expr::Op(Perform {
                            op: Operation::DefineVariable,
                            args: vec![
                                Expr::Const(Const::Symbol(var)),
                                Expr::Register(Register::Val),
                                Expr::Register(Register::Env),
                            ],
                        }),
                    ),
                    Instruction::Assign(target, Expr::Const(Const::Symbol("ok".to_string()))),
                ],
            ),
        ),
    )
}

fn compile_if(mut exp: Vec<UMPL2Expr>, target: Register, linkage: Linkage) -> InstructionSequnce {
    let t_branch = make_label_name("true-branch".to_string());
    let f_branch = make_label_name("false-branch".to_string());
    let after_if = make_label_name("after-if".to_string());
    let consequent_linkage = if linkage == Linkage::Next {
        Linkage::Label(after_if.clone())
    } else {
        linkage.clone()
    };
    if exp.len() != 4 {
        panic!()
    }

    let p_code = compile(exp.remove(1), Register::Val, Linkage::Next);

    let a_code = {
        let mut a = compile(exp.remove(1), target, consequent_linkage);
        a.instructions.insert(0, Instruction::Label(t_branch));
        a
    };

    let c_code = {
        let mut c = compile(exp.remove(1), target, linkage);
        c.instructions
            .insert(0, Instruction::Label(f_branch.clone()));
        c.instructions.push(Instruction::Label(after_if));
        c
    };
    preserving(
        hashset!(Register::Continue),
        p_code,
        append_instruction_sequnce(
            InstructionSequnce::new(
                hashset!(Register::Val),
                hashset!(),
                vec![
                    Instruction::Test(Perform {
                        op: Operation::False,
                        args: vec![Expr::Register(Register::Val)],
                    }),
                    Instruction::Branch(f_branch),
                ],
            ),
            parallel_instruction_sequnce(a_code, c_code),
        ),
    )
}

fn compile_seq(seq: Vec<UMPL2Expr>, target: Register, linkage: Linkage) -> InstructionSequnce {
    let size = seq.len();
    seq.into_iter()
        .enumerate()
        .map(move |(i, exp)| {
            if size == i - 1 {
                compile(exp, target, linkage.clone())
            } else {
                compile(exp, target, Linkage::Next)
            }
        })
        .reduce(|a, b| preserving(hashset!(), a, b))
        .unwrap_or_else(empty_instruction_seqeunce)
}

fn compile_lambda(
    mut lambda: Vec<UMPL2Expr>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    lambda.remove(0);
    let proc_entry = make_label_name("entry".to_string());
    let after_lambda = make_label_name("after_lambda".to_string());
    let lambda_linkage = if linkage == Linkage::Next {
        Linkage::Label(after_lambda.clone())
    } else {
        linkage
    };
    let mut first_inst = end_with_linkage(
        lambda_linkage,
        InstructionSequnce::new(
            hashset!(Register::Env),
            hashset!(target),
            vec![Instruction::Assign(
                target,
                Expr::Op(Perform {
                    op: Operation::MakeCompiledProcedure,
                    args: vec![
                        Expr::Label(proc_entry.clone()),
                        Expr::Register(Register::Env),
                    ],
                }),
            )],
        ),
    );
    let body = compile_lambda_body(lambda, proc_entry);
    first_inst.instructions.extend(body.instructions);
    first_inst
        .instructions
        .push(Instruction::Label(after_lambda));
    first_inst
}

fn compile_lambda_body(mut lambda: Vec<UMPL2Expr>, proc_entry: String) -> InstructionSequnce {
    let formals = {
        let arg_c = match &lambda[0] {
            // UMPL2Expr::Number(n) => (n, "".into()),
            UMPL2Expr::Application(a) => match a.as_slice() {
                [UMPL2Expr::Number(n), UMPL2Expr::Ident(s)]
                    if ["+".into(), "*".into()].contains(s) =>
                {
                    *n + 1.0
                }

                [UMPL2Expr::Number(n)] => *n,
                _ => todo!("self function should return result so self can error"),
            },
            _ => todo!("self function should return result so self can error"),
        }
        .trunc() as u64;
        lambda.remove(0);
        (0..arg_c)
            .map(|i| Expr::Const(Const::Symbol(format!("'{i}'"))))
            .rfold(Expr::Const(Const::Empty), |a, b| {
                Expr::Const(Const::List(Box::new(b), Box::new(a)))
            })
    };

    append_instruction_sequnce(
        InstructionSequnce::new(
            hashset!(Register::Env, Register::Proc, Register::Argl),
            hashset!(Register::Env),
            vec![
                Instruction::Label(proc_entry),
                Instruction::Assign(
                    Register::Env,
                    Expr::Op(Perform {
                        op: Operation::CompiledProcedureEnv,
                        args: vec![Expr::Register(Register::Proc)],
                    }),
                ),
                Instruction::Assign(
                    Register::Env,
                    Expr::Op(Perform {
                        op: Operation::ExtendEnvoirnment,
                        args: vec![
                            formals,
                            Expr::Register(Register::Argl),
                            Expr::Register(Register::Env),
                        ],
                    }),
                ),
            ],
        ),
        compile_seq(lambda, Register::Val, Linkage::Return),
    )
}

fn compile_application(
    exp: Vec<UMPL2Expr>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    let proc_code = compile(exp[0].clone(), Register::Proc, Linkage::Next);
    let operand_codes = exp[1..]
        .iter()
        .map(|exp| compile(exp.clone(), Register::Val, Linkage::Next))
        .collect();
    preserving(
        hashset!(Register::Continue, Register::Env),
        proc_code,
        preserving(
            hashset!(Register::Proc, Register::Continue),
            construct_arg_list(operand_codes),
            compile_procedure_call(target, linkage),
        ),
    )
}

fn make_label_instruction(label: Label) -> InstructionSequnce {
    InstructionSequnce::new(hashset!(), hashset!(), vec![Instruction::Label(label)])
}

fn make_intsruction_sequnce(
    needs: HashSet<Register>,
    modifies: HashSet<Register>,
    instructions: Vec<Instruction>,
) -> InstructionSequnce {
    InstructionSequnce::new(needs, modifies, instructions)
}

fn compile_procedure_call(target: Register, linkage: Linkage) -> InstructionSequnce {
    let primitive_branch = make_label_name("primitive-branch".to_string());
    let compiled_branch = make_label_name("compiled-branch".to_string());
    let after_call = make_label_name("after-call".to_string());
    let compiled_linkage = if linkage == Linkage::Next {
        Linkage::Label(after_call.clone())
    } else {
        linkage.clone()
    };
    append_instruction_sequnce(
        InstructionSequnce::new(
            hashset!(Register::Proc),
            hashset!(),
            vec![
                Instruction::Test(Perform {
                    op: Operation::PrimitiveProcedure,
                    args: vec![Expr::Register(Register::Proc)],
                }),
                Instruction::Branch(primitive_branch.clone()),
            ],
        ),
        parallel_instruction_sequnce(
            append_instruction_sequnce(
                make_label_instruction(compiled_branch),
                compile_proc_appl(target, compiled_linkage),
            ),
            append_instruction_sequnce(
                make_label_instruction(primitive_branch),
                append_instruction_sequnce(
                    end_with_linkage(
                        linkage,
                        make_intsruction_sequnce(
                            hashset!(Register::Proc, Register::Argl),
                            hashset!(target),
                            vec![Instruction::Assign(
                                target,
                                Expr::Op(Perform {
                                    op: Operation::ApplyPrimitiveProcedure,
                                    args: vec![
                                        Expr::Register(Register::Proc),
                                        Expr::Register(Register::Argl),
                                    ],
                                }),
                            )],
                        ),
                    ),
                    make_label_instruction(after_call),
                ),
            ),
        ),
    )
}

fn compile_proc_appl(target: Register, compiled_linkage: Linkage) -> InstructionSequnce {
    match (target, compiled_linkage) {
        (Register::Val, Linkage::Return) => make_intsruction_sequnce(
            hashset!(Register::Proc, Register::Continue),
            all_regs(),
            vec![
                Instruction::Assign(
                    Register::Val,
                    Expr::Op(Perform {
                        op: Operation::CompiledProcedureEntry,
                        args: vec![Expr::Register(Register::Proc)],
                    }),
                ),
                Instruction::Goto(Goto::Register(Register::Val)),
            ],
        ),
        (Register::Val, Linkage::Label(l)) => make_intsruction_sequnce(
            hashset!(Register::Proc),
            all_regs(),
            vec![
                Instruction::Assign(Register::Continue, Expr::Label(l)),
                Instruction::Assign(
                    Register::Val,
                    Expr::Op(Perform {
                        op: Operation::CompiledProcedureEntry,
                        args: vec![Expr::Register(Register::Proc)],
                    }),
                ),
                Instruction::Goto(Goto::Register(Register::Val)),
            ],
        ),
        (_, Linkage::Next) => unreachable!(),
        (_, Linkage::Return) => panic!("return linkage, target not val -- COMPILE {target}"),
        (_, Linkage::Label(l)) => {
            let proc_return = make_label_name("proc-return".to_string());
            make_intsruction_sequnce(
                hashset!(Register::Proc),
                all_regs(),
                vec![
                    Instruction::Assign(Register::Continue, Expr::Label(proc_return.clone())),
                    Instruction::Assign(
                        Register::Val,
                        Expr::Op(Perform {
                            op: Operation::CompiledProcedureEntry,
                            args: vec![Expr::Register(Register::Proc)],
                        }),
                    ),
                    Instruction::Label(proc_return),
                    Instruction::Assign(target, Expr::Register(Register::Val)),
                    Instruction::Goto(Goto::Label(l)),
                ],
            )
        }
    }
}

fn all_regs() -> HashSet<Register> {
    hashset!(
        Register::Continue,
        Register::Argl,
        Register::Env,
        Register::Proc,
        Register::Val
    )
}

fn add_to_argl(inst: InstructionSequnce) -> InstructionSequnce {
    preserving(
        hashset!(Register::Argl),
        inst,
        InstructionSequnce::new(
            hashset!(Register::Val, Register::Argl),
            hashset!(Register::Argl),
            vec![Instruction::Assign(
                Register::Argl,
                Expr::Op(Perform {
                    op: Operation::Cons,
                    args: vec![
                        Expr::Register(Register::Val),
                        Expr::Register(Register::Argl),
                    ],
                }),
            )],
        ),
    )
}

fn construct_arg_list(operand_codes: Vec<InstructionSequnce>) -> InstructionSequnce {
    operand_codes.into_iter().map(add_to_argl).rev().fold(
        InstructionSequnce::new(
            hashset!(),
            hashset!(Register::Argl),
            vec![Instruction::Assign(
                Register::Argl,
                Expr::Const(Const::Empty),
            )],
        ),
        |a, b| preserving(hashset!(Register::Env), a, b),
    )
}

impl From<UMPL2Expr> for Expr {
    fn from(value: UMPL2Expr) -> Self {
        match value {
            UMPL2Expr::Bool(b) => match b {
                crate::ast::Boolean::False => Self::Const(Const::Boolean(false)),
                crate::ast::Boolean::True => Self::Const(Const::Boolean(true)),
                crate::ast::Boolean::Maybee => Self::Op(Perform {
                    op: Operation::RandomBool,
                    args: vec![],
                }),
            },
            UMPL2Expr::Number(n) => Self::Const(Const::Number(n)),
            UMPL2Expr::String(s) => Self::Const(Const::String(s.to_string())),
            UMPL2Expr::Ident(i) => Self::Const(Const::Symbol(i.to_string())),
            UMPL2Expr::Application(a) => a
                .into_iter()
                .map(Into::into)
                .rfold(Self::Const(Const::Empty), |a, b| {
                    Self::Const(Const::List(Box::new(b), Box::new(a)))
                }),
            UMPL2Expr::Label(l) => Self::Label(l.to_string()),
            UMPL2Expr::FnParam(i) => Self::Const(Const::Symbol(format!("'{i}'"))),
        }
    }
}
