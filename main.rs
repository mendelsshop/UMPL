#![allow(unu)]


fn main() {
    let cons = make_cons();
    cons(5, 6); //
}

fn make_cons() -> impl Fn(i32, i32) -> Box<dyn Fn(i8) -> i32> {
    move |x, y| {
        Box::new(move |z: i8| { 
            match z {
                0 => x,
                1 => y,
                _ => panic!(""),

            }
        })
    }
}