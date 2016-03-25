fn main () {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "The quick brown fox",
             verb = "jumps over");

    println!("{} of {:b} people know binary the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    // struct Structure(i32);
    // println!("This struct '{}' won't print...", Structure(3));

// Not right...
    // let pi = format!("{:.*}", 3, 22/7);
    //println!("Pi is roughly {}", format!("{:.*}", 3, 22/7));

    // let pi: i32 = 22 / 7;
    // println!("Pi is roughly {:.3}", 22 / 7 as i32);

    // Fuck wtf

    // let pi = format!("{:.*}", 3, 22/7);
    // assert_eq!("3.143", pi);

    println!("PI is roughly {:.3}", 22/7);

}
