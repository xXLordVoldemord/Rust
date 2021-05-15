fn main() {
    



    let condition = true;
    
    let test1 = if condition{5} else{3};
    println!("test1 ist: {}", test1);




    let iftest = 3;

    if iftest !=5{
        println!("test ist kleiner 5");
    } else {
        println!("test ist größer gleich 5");
    }

    another_function(7.2);
    println!("ganz doll");
}
fn another_function(y: f64){
    println!("{}",y);

    let x = 6;
    let y = {
        let x =3;
        x+1
    };

    println!("der wert von y ist: {}", y);

    let x = five();
    println!("the value of x is : {}", x);

    let x = plus_one(5);

    println!("Methofe funktioniert; x ist: {}", x);

}

fn five() -> f32 {
    5.0


}

fn plus_one(x: i32) -> i32 {
    x+1
}