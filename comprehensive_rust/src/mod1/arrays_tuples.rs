pub mod arrays_tuples {
    pub fn sample() {
        // Arrays and Tuples
        let mut a: [i64; 10] = [42; 10];
        a[5] = 0;

        let mut t: (i64, bool) = (9, true);
        println!("{:?}", a);
        println!("{:?}", t);
        t.0 = 7;
        println!("{:?}", t);
    }
}
