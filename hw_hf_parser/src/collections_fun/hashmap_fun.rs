use std::collections::HashMap;

pub fn exp_with_hashmap() -> () {

    // Initialize Empty
    let mut scores = HashMap::new(); // noticed mut

    // Add Elements - not challenges of strings
    scores.insert("Blue", 10);
    scores.insert("Green", 10);
    println!("{:?}", scores);
    
    // Read 
    // read with index, not can't set with index
    println!("read with index {}", scores["Blue"]); 
    // get returns an Option (Enum)
    // https://doc.rust-lang.org/std/option/enum.Option.html#
    let blue_option = scores.get("Blue"); // returns option & optional
    let black_option = scores.get("Black"); // returns option & optional
    println!("opition-{:?}-{:?}", blue_option, black_option);
    //pub fn get<Q>(&self, k: &Q) -> Option<&V>
    //get returuns an option with a reference to value
    assert_eq!(blue_option, Some(&10)); // <-- passes
    // assert_eq!(blue_option, Some(10)); // <-- complains of type mismatch
    
    // Enums have methods to get values in one line
    println!("raw unwrap-{:?}", blue_option.unwrap_or(&0)); //0 fails, mismatch type
    // pub fn unwrap_or(self, default: T) -> T
    println!("raw copied-{:?}", blue_option.copied());
    // pub fn copied(self) -> Option<T>
    println!("raw copied unrwapped-{:?}", blue_option.copied().unwrap_or(0));
    
    // Iterate - Borrowing
    for (k, v) in &scores{
        assert_eq!(v, &10);
        assert_eq!(*v, 10);
        println!("from loop {k}: {v}")
    };
    // Iterate - Moving
    for (k, v) in scores{
        //assert_eq!(v, &10);
        assert_eq!(v, 10);
        println!("from loop {k}: {v}")
    };
    // gets an error
    //println!("read with index {}", scores["Blue"]);

    // Load using String on Heap that moves references to dictionary
    let mut scores = HashMap::new(); 
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 10);
    assert_eq!(scores["Blue"], 10); // this is ok

    // Updating - if exists - overwrite, ignore, change
    // Updating - overwrite
    //scores.insert("Blue", 20); could not use literal here -  
    //error expected `String`, found `&str` <-- this is interesting
    scores.insert(String::from("Blue"), 25); 
    println!("{:?}", scores);
    // Updating - ignore or insert leverages Entry enum
    // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/collections/hash_map/enum.Entry.html
    // pub fn entry(&mut self, key: K) -> Entry<'_, K, V>
    // pub enum Entry<'a, K: 'a, V: 'a> {
    //      Occupied(OccupiedEntry<'a, K, V>),
    //      Vacant(VacantEntry<'a, K, V>),
    // }
    // pub fn or_insert(self, default: V) -> &'a mut V
    let v = scores.entry(String::from("Blue")).or_insert(50);
    assert_eq!(*v, 25);
    let v = scores.entry(String::from("Yellow")).or_insert(50);
    assert_eq!(*v, 50);
    println!("{:?}", scores);
    // Updating - amend value leverages Entry enum
    let v = scores.entry(String::from("Yellow")).or_insert(50);
    assert_eq!(*v, 50);
    *v += 50; // return mutable reference
    assert_eq!(*v, 100);
    println!("{:?}", scores);
    
}