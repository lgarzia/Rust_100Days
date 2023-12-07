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
    let mut v = vec![String::from("1"), String::from("2"), String::from("3")];  
    //let _two = v[2]; // since integers just copied there's no difference between using & or not
    //println!("vector-{:?}-{_two}", v);
    let _two = &v[0];
    println!("vector-{:?}-{_two}", v);
    // get returns an Option enum (Some, None)
    //pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
    // https://doc.rust-lang.org/std/option/enum.Option.html
    let _out_of_range = v.get(4).cloned().unwrap_or(String::from("out of range"));
    // cloned required to get from & -> Value in order to leverage unwrap_or
    println!("capture-{_out_of_range}");
    //let us_panic = &v[100]; 
    // rust ownership rules still apply to elements within
    // can't have mutable and immutable references in the same scope
    let _first = &v[0]; // immutable borrow
    //v.push("6".to_string()); //mutable borrow <-- complains
    // vectors put elements next to each other in memory
    // appending may requires allocating new memory
    // coluld leave reference to deallocated memory
    // Iterating over vector:
    for i in &mut v {  //&v versus &mut - either way prevents vector adding/removing records
        *i = "m".to_string();  //dereference
        println!("{i}")
    }
    // use enums and structs more complicated datastructures
    struct Point {
        x: i32,
        y: i32,
    }
    let mut points:Vec<Point> = Vec::new();
    let point1 = Point { x: 10, y: 20 };
    let point2 = Point { x: 30, y: 40 };
    points.push(point1);
    points.push(point2);
    let first_point = &points[0]; //points[0] will return a copy, &points[0] returns a reference
    println!("First point: x = {}, y = {}", first_point.x, first_point.y);
    // use enum to trick to hold different data types
    #[derive(Debug)]
    enum Cell {
        Int(i32), 
        Float(f64),
        Text(String),
    }
    let row = vec![Cell::Int(3), Cell::Text("Blue".to_string()), Cell::Float(10.12)];
    for r in &row {
        println!("{:?}", r)
    }
    // Iterators https://doc.rust-lang.org/std/slice/struct.Iter.html
    // provides functional language and iterator constructs
    let primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    let mut numbers = vec![1, 2, 3, 4, 5];
    // Map the numbers to their squares using a function
    fn square_x(x: &i32) -> i32 {
        x * x
    }
    let squared_numbers: Vec<i32> = numbers.iter().map(square_x).collect();
    println!("Squared numbers: {:?}", squared_numbers);

    // range selection
    let _x: Vec<i32> = numbers[0..1].to_vec();
    let _y:&[i32] = &numbers[0..2];
    let _z:&mut[i32] = &mut numbers[0..2];

    //len versus capacity
    println!("{}", numbers.len());
    println!("{}", numbers.capacity());

    //https://doc.rust-lang.org/std/primitive.slice.html
    //https://doc.rust-lang.org/std/slice/index.html
    //A dynamically-sized view into a contiguous sequence, [T]. 
    //Contiguous here means that elements are laid out so that every element is the same distance from its neighbors.

    //reference to a slice is a FAT pointer
    // function that works on a slice
    fn _print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }
    // print(&vector[0..2]); --> &str, &[T] is slices
    // methods on slice, available on vectors
}