fn main() {
    
    let mut b = String::from("hello");

    b.push_str(", wolrd!"); // push_str() appends lateral to a String Tanga
    //println!("{}", b);

    let b2 = b.clone();
    println!("b = {} b2 = {}",b, b2);

    takes_ownership(b);


    let x =5;
    makes_copy(x);

}

fn takes_ownership(some_string: String) {
    println!("der string ist {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
