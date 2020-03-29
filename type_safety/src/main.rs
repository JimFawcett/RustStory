fn main() {
    let mut v = Vec::<i32>::with_capacity(3);
    v.push(1);
    v.push(2);
    v.push(3);
    print!("\n  v capacity = {}", v.capacity());

    let r1 = &v[1];
    print!("\n  address of v[1] = {:?}", &v[1] as *const i32);
    print!("\n  address of r1   = {:?}", r1 as *const i32);
    
    v.push(4);  // fails to compile, can't mutate while borrowed
    print!("\n  address of v[1] = {:?}", &v[1] as *const i32);
    //print!("\n  address of r1   = {:?}", r1 as *const i32);
    
    println!("\n\n  Hello, Ownership!\n");
}
