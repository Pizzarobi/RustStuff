fn main() {
    let y = another_function(3,2);
    println!("The value of y is {}",y);

    if y < 5{
        println!("1");
    }else if y == 5 {
        println!("2");
    } else {
        println!("3");
    }
}

fn another_function(x: i32, y: i32) -> i32{
    println!("The value of x is {} and y is {}",x,y);
    x+y
}