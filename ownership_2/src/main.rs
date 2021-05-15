fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);


    let a1 = String::from("hello");

    let (a2, len) = calculate_length(a1);
    println!("The length of {} is {}", a2, len);

}

fn calculate_length(s:String) -> (String, usize){

    let length = s.len();
    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back (a_string: String) ->String{
    a_string
}
