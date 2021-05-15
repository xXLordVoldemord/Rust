fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("conditon was false");
    }

    if number !=0 {
        println!("number is sth else than zero");
    }

    if number % 4 == 0{
        println!("number is dvisible by 4"); 
    } else if number % 3 == 0 {
        println!("number is dvisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisble by 2");
    }


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter *2;
        }


    };
    println!("the result is {}", result);


    let mut fucker = 3;
    
    while fucker != 0 {
        println!("tiger TIGER");

        fucker -= 1;
    }

    println!("Liftoff!!!");

    let summe = iterate_shit();

    println!("das ergebnis ist {}", summe);



    let a = [10,20,30,40,50];

    for penis in a.iter(){
        println!("der wert ist {}", penis);
    }

    for b in (1..4).rev() {
        println!("try out {}", b);
    }

}

fn iterate_shit () -> f32 {
    let goal = 100.0;
    let mut counter = 0;
    let mut start = 50.0;
    let mut sum = 0.0;

    while sum < goal {
        
        sum = sum + start;
        start = start/2.0;


        counter += 1;

        println!("counter {}", counter);
        println!("summe {}", sum);
    };    
    return sum;
}



    