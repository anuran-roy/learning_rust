pub mod vectors {
    pub fn sample() {
        let mut vector: Vec<i16> = vec![1, 2, 3];
        println!("{:?}", vector.len() / 2);
        vector.push(4);
        let mut vector2: Vec<i16> = vec![5, 6, 7];
        println!("{:?}", vector2.len());
        vector.append(&mut vector2);
        println!("{:?}", vector.len());
        println!("\nUsing vector.iter()...");
        for i in vector.iter() {
            print!("{},", i);
        }
        println!("\nUsing traditional loop...");
        for i in 0..vector.len() {
            print!("{},", vector[i]);
        }
        println!("\n{:?}", vector2.len());
    }
}
