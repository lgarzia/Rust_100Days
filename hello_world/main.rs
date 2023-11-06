fn main(){
    println!("Hello, world!");

    let mut x = 10; 
    let r1 = &x; 
    let r2 = &x; 
    x += 10; // error: cannot assign to `x` because it is borrowed
    let m = &mut x; // error: cannot borrow `x` as mutable because it is also borrowed as immutable
    println!("{} {} {}", r1, r2, m);
    

}