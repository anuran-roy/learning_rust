pub mod scope_n_shadowing {
    pub fn sample() {
        let a = 10;
        println!("before: {a}");

        {
            let a = "hello";
            println!("inner scope: {a}");

            let a = true;
            println!("shadowed in inner scope: {a}");
        }

        println!("after: {a}");
    }
}
