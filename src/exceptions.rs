// use libc::c_char;
use std::ffi::{CStr, c_char};
use std::mem;
use std::ptr;

use inkwell::{
    context::Context,
    module::Module,
    AddressSpace, execution_engine::ExecutionEngine,
};

use crate::codegen::{Function, Object};
// use crate::error::RuntimeError;

// use unlisp_internal_macros::runtime_fn;

const JMP_BUF_SIZE: usize = mem::size_of::<u32>() * 40;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RuntimeError(String);

impl RuntimeError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

type JmpBuf = [i8; JMP_BUF_SIZE];

#[export_name = "glob_jmp_buf"]
#[no_mangle]
#[used]
static mut GLOB_JMP_BUF: JmpBuf = [0; JMP_BUF_SIZE];

#[export_name = "err_msg_ptr"]
#[no_mangle]
#[used]
static mut ERR_MSG_PTR: *mut i8 = ptr::null_mut();

unsafe fn jmp_buf_ptr(buf: &mut JmpBuf) -> *mut i8 {
    &mut buf[0] as *mut i8
}

unsafe fn glob_jmp_buf_ptr() -> *mut i8 {
    jmp_buf_ptr(&mut GLOB_JMP_BUF)
}

extern "C" {
    pub fn setjmp(buf: *mut i8) -> i32;
    pub fn longjmp(buf: *const i8) -> !;
}

pub unsafe fn run_with_global_ex_handler<F: FnOnce() -> Object>(
    f: F,
) -> Result<Object, RuntimeError> {
    let mut prev_handler: JmpBuf = mem::zeroed();
    ptr::copy_nonoverlapping(
        glob_jmp_buf_ptr(),
        jmp_buf_ptr(&mut prev_handler),
        JMP_BUF_SIZE,
    );

    let result = if setjmp(glob_jmp_buf_ptr()) == 0 {
        Ok(f())
    } else {
        Err(RuntimeError::new((*(ERR_MSG_PTR as *mut String)).clone()))
    };

    ptr::copy_nonoverlapping(
        jmp_buf_ptr(&mut prev_handler),
        glob_jmp_buf_ptr(),
        JMP_BUF_SIZE,
    );

    result
}

pub unsafe fn raise_error(msg: String) -> ! {
    ERR_MSG_PTR = Box::into_raw(Box::new(msg)) as *mut i8;
    let buf = glob_jmp_buf_ptr();
    longjmp(buf)
}
pub fn global_ex_handler_gen_def<'a, 'ctx>(ctx: &'ctx Context, module: &'a Module<'ctx>, jit: ExecutionEngine<'ctx>) {
    jit.add_global_mapping( &module.add_function(
        "global_error_handling",
        ctx.i32_type().fn_type(
            &[module
                .get_struct_type("unlisp_rt_function")
                .unwrap()
                
                .ptr_type(AddressSpace::default())
                .into()],
            false,
        ),
        None,
    ), unlisp_rt_run_with_global_ex_handler as usize );
}
pub fn gen_defs<'a, 'ctx>(ctx: &'ctx Context, module: &'a Module<'ctx>,jit: ExecutionEngine<'ctx>) {
    sjlj_gen_def(ctx, module, jit.clone());
    global_ex_handler_gen_def(ctx, module, jit)
//     raise_arity_error_gen_def(ctx, module);
//     raise_undef_fn_error_gen_def(ctx, module);
}

pub fn sjlj_gen_def<'a, 'ctx>(ctx: &'ctx Context, module: &'a Module<'ctx>,jit: ExecutionEngine<'ctx>) {
    let i32_ty = ctx.i32_type();

    let buf_ty = ctx.opaque_struct_type("setjmp_buf");
    let int32_arr_ty = i32_ty.array_type(40);
    buf_ty.set_body(&[int32_arr_ty.into()], false);
    // has to be looked up through module, to avoid renaming
    let buf_ptr_ty = module
        .get_struct_type("setjmp_buf")
        .unwrap()
        .ptr_type(AddressSpace::default());
    let void_ty = ctx.void_type();
    let sj_fn_ty = i32_ty.fn_type(&[buf_ptr_ty.into()], false);
    let lj_fn_ty = void_ty.fn_type(&[buf_ptr_ty.into(), i32_ty.into()], false);
    jit.add_global_mapping(
        &module.add_function(
            "setjmp",
            sj_fn_ty,
            None,
        ),
        setjmp as usize,
    );
    jit.add_global_mapping(
        &module.add_function(
            "longjmp",
            lj_fn_ty,
            None,
        ),
        longjmp as usize,
    );
}

pub unsafe fn raise_cast_error(from: String, to: String) -> ! {
    let msg = format!("cannot cast {} to {}", from, to);

    raise_error(msg)
}

// #[runtime_fn]
pub unsafe extern "C" fn unlisp_rt_run_with_global_ex_handler(f: *mut Function) -> i32 {
    let invoke_fn: unsafe extern "C" fn(*const Function) -> Object =
        mem::transmute((*f).invoke_f_ptr);
    match run_with_global_ex_handler(|| invoke_fn(f)) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("runtime error: {}", e);
            1
        }
    }
}

// #[runtime_fn]
pub unsafe extern "C" fn unlisp_rt_raise_arity_error(
    name: *const c_char,
    _expected: u64,
    actual: u64,
) -> ! {
    let name_str = if name != ptr::null() {
        CStr::from_ptr(name).to_str().unwrap()
    } else {
        "lambda"
    };

    let msg = format!(
        "wrong number of arguments ({}) passed to {}",
        actual, name_str
    );

    raise_error(msg);
}

// #[runtime_fn]
pub unsafe extern "C" fn unlisp_rt_raise_undef_fn_error(name: *const c_char) -> ! {
    let name_str = CStr::from_ptr(name).to_str().unwrap();

    let msg = format!("undefined function {}", name_str);

    raise_error(msg);
}
