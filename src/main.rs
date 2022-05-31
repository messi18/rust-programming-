mod restaurant;
mod ch4;
mod ch5;
mod ch10;
mod ch8;
mod ch12;
mod ch13;
mod ch15;
mod ch16;

use restaurant::*;
fn main() {
    //let mut x = 5;
    //println!("The value of x is : {}", x);
    let x = 6;
    println!("The value of x is : {}", x);
    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //println!("The value of const is : {}", THREE_HOURS_IN_SECONDS);
    let meal:Breakfast = eat_at_restaurant();
    println!("meal:{}", meal)
}
