use std::ffi::CStr;
#[inline(never)]
#[no_mangle]
pub extern "C" fn extract_num(object: Object) -> f64 {
    if object.kind != 1 {
        panic!()
    }
    unsafe {
        let i  = object.tag;
        println!("{}", i.bool);
        return i.number;
    }
    // return 1;
}
// #[used]
// static ___USED_UNLISP_RT_INT_FROM_OBJ: extern "C" fn(o: Object) -> i32 = extract_num;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Object {
    tag: UntaggedObject,
    kind: i8,

    //     bool: bool,
    // string: *const i8,
    // number: f64
    // lambda: *const Function
}


#[derive(Clone, Copy)]
#[repr(C)]
pub union UntaggedObject {
    string: *const i8,
        bool: bool,
    number: f64,
    // lambda: *mut Function,
    lambda: *const i8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum TyprIndex {
    String = 0,
    Number = 1,
    Boolean = 2,
    Lambda = 3,
}
fn main() {}