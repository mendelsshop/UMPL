fn main() {
    let yzzz = 1;
    let ooo = 2;
    let u = || yzzz + ooo;
    
    let z = |a| {
      let xyz =   yzzz + ooo;
    //   let yy= || xyz + ooo + a;

    //   ooo;
    //   yy();
        xyz
    };
    z(1);
    u();
    
}