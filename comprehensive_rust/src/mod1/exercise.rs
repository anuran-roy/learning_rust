pub mod q1 {
    // pub fn sample(a: u8, b: i16) -> i16 { // No implicit conversion, unless there is a standard or user-defined From<T> and Into<T> implementation.

    // NOTE: Error message on trying implicit conversion:
    //
    //  error[E0308]: mismatched types
    // --> src/implicit.rs:3:20
    // |
    // 3 |         return a + b;
    // |                    ^ expected `u8`, found `i16`

    // error[E0308]: mismatched types
    // --> src/implicit.rs:3:16
    // |
    // 2 |     pub fn sample(a: u8, b: i16) -> i16 {
    // |                                     --- expected `i16` because of return type
    // 3 |         return a + b;
    // |                ^^^^^ expected `i16`, found `u8`
    // |
    // help: you can convert a `u8` to an `i16`
    // |
    // 3 |         return (a + b).into();
    // |                +     ++++++++

    // error[E0277]: cannot add `i16` to `u8`
    // --> src/implicit.rs:3:18
    // |
    // 3 |         return a + b;
    // |                  ^ no implementation for `u8 + i16`
    // |
    // = help: the trait `Add<i16>` is not implemented for `u8`
    // = help: the following other types implement trait `Add<Rhs>`:
    //             <&'a f32 as Add<f32>>
    //             <&'a f64 as Add<f64>>
    //             <&'a i128 as Add<i128>>
    //             <&'a i16 as Add<i16>>
    //             <&'a i32 as Add<i32>>
    //             <&'a i64 as Add<i64>>
    //             <&'a i8 as Add<i8>>
    //             <&'a isize as Add<isize>>
    //         and 48 others

    // Some errors have detailed explanations: E0277, E0308.
    // For more information about an error, try `rustc --explain E0277`.
    // error: could not compile `comprehensive_rust` due to 3 previous errors
    pub fn sample(a: u8, b: i16) -> i16 {
        return i16::from(a) + b; // Convert a from u8 to i16
    }
}

pub mod q2 {
    pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
        let mut matrix2 = [[0; 3]; 3];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix2[j][i] = matrix[i][j];
            }
        }

        return matrix2;
    }

    pub fn pretty_print(matrix: &[[i32; 3]; 3]) {
        // unimplemented!()
        for i in 0..matrix.len() {
            print!("| ");
            for j in 0..matrix[0].len() {
                print!("{} ", matrix[i][j]);
            }
            print!("|");
            println!();
        }
    }

    pub fn sample() {
        let matrix = [
            [101, 102, 103], // <-- the comment makes rustfmt add a newline
            [201, 202, 203],
            [301, 302, 303],
        ];

        println!("matrix:");
        pretty_print(&matrix);

        let transposed = transpose(matrix);
        println!("transposed:");
        pretty_print(&transposed);
    }
}
