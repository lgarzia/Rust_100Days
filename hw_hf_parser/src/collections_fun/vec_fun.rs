// https://doc.rust-lang.org/std/vec/struct.Vec.html#
// https://doc.rust-lang.org/std/macro.vec.html

pub fn vec_fun() -> () {
    let mut v: Vec<i32> = Vec::new(); // part of prelude
    let mut v2 = vec![1, 2, 3]; // https://doc.rust-lang.org/std/macro.vec.html

    // Operations --> add a value
    println!("in vec_fun");
    v.push(5); 
    v2.insert(0, 6);
    v.append(&mut v2); // &mut - requires mutable references
    // pub fn append(&mut self, other: &mut Vec<T, A>)
    println!("vector-{:?}", v);
    // reading elements
    // index, get method
    let v = vec![String::from("1"), String::from("2"), String::from("3")];  
    //let _two = v[2]; // since integers just copied there's no difference between using & or not
    //println!("vector-{:?}-{_two}", v);
    let _two = &v[0];
    println!("vector-{:?}-{_two}", v);
    // get returns an Option enum (Some, None)
    //pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    // https://doc.rust-lang.org/std/option/enum.Option.html
    let _out_of_range = v.get(4).cloned().unwrap_or(String::from("pain, all I feel"));
    // cloned required to get from & -> Value in order to leverage unwrap_or
    
    println!("capture-{_out_of_range}");

}