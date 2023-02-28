fn main() {
    // let y = another_function(3,2);
    // println!("The value of y is {}",y);

    // if y < 5{
    //     println!("1");
    // }else if y == 5 {
    //     println!("2");
    // } else {
    //     println!("3");
    // }

    reverse_countdown();
}

fn another_function(x: i32, y: i32) -> i32{
    println!("The value of x is {} and y is {}",x,y);
    x+y
}

fn iterate_through_array(){
    let a = [5,3,2,5,6];

    for element in a {
        println!("val: {element}");
    }
}

fn reverse_countdown(){
    for number in (1..5).rev() {
        println!("{number}!");
    }
    println!("Liftoff!!!");
}