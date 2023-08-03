// #![allow(unu)]
use std::rc::Rc;
#[derive(Debug, Clone)]
struct t {
    v: i32,
    p: Rc<str>,
}
fn main() {
   let cons= move |x, y| {
        move |z: i8| { 
            match z {
                0 => x,
                1 => y,
                _ => panic!(""),

            }
        }
    };
    let v = cons(t{v:1, p: "bb".into()}, t{v:2, p: "aa".into()}); //
    v(1);
}

// fn make_cons() -> impl Fn(i32, i32) -> Box<dyn Fn(i8) -> i32> {
//     let k=t {v: 5, p: "hello".into()};
//     move |x, y| {
//         let k =k.clone();
//         Box::new(move |z: i8| { 
//             match z {
//                 0 => x,
//                 1 => y,
//                 _ => panic!("{}", k.v),

//             }
//         })
//     }
// }