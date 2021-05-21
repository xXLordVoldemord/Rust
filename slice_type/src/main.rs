fn main() {

    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let len = s.len();

    let test = &s[..len];


    let method = first_word(&s);

    println!("metod is equal to {}", method);



println!("value of method is: {}", method);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//fn second_word(s: &String) -> (usize, usize){

//}