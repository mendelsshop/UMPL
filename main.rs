fn main() {
    let cons = |x, y| {
        move |z: i32| {
            if z == 0 {
                x
            } else if z == 1 {
                y
            } else {
                panic!("Invalid")
            }

        }
    };
    let x = cons(5, 6);
    x(0);
}