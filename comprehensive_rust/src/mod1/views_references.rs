pub mod views_references {
    pub fn sample() {
        // References
        let mut x = 10;
        let ref_x: &mut i32 = &mut x;
        *ref_x = 20;
        println!("x: {x}");

        // Dangling References
        // let ref_x: &i32;
        // {
        //     let x: i32 = 10;
        //     ref_x = &x;
        // }
        // println!("ref_x: {ref_x}");

        // Views
        let mut a: [i64; 10] = [42; 10];
        let mut s = &mut a[2..4];
        println!("s: {:?}", s);
        a[3] = 2;
        s = &mut a[2..4]; // Error without this line
                          // Error message:

        //     error[E0503]: cannot use `a` because it was mutably borrowed
        //   --> src/main.rs:30:5
        //    |
        // 28 |     let mut s = &mut a[2..4];
        //    |                      - borrow of `a` occurs here
        // 29 |     println!("s: {:?}", s);
        // 30 |     a[3] = 2;
        //    |     ^^^^ use of borrowed `a`
        // 31 |     println!("s: {:?}", s);
        //    |                         - borrow later used here

        // error[E0506]: cannot assign to `a[_]` because it is borrowed
        //   --> src/main.rs:30:5
        //    |
        // 28 |     let mut s = &mut a[2..4];
        //    |                      - borrow of `a[_]` occurs here
        // 29 |     println!("s: {:?}", s);
        // 30 |     a[3] = 2;
        //    |     ^^^^^^^^ assignment to borrowed `a[_]` occurs here
        // 31 |     println!("s: {:?}", s);
        //    |                         - borrow later used here
        println!("s: {:?}", s);
    }
}
