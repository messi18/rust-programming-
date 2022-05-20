use std::io;

#[cfg(test)]
mod test {
    use crate::ch4::*;
    #[test]
    fn test_3_1() {

        run3_1();
    }
    #[test]
    fn test_3_2() {
        run3_2();
    }
    #[test]
    fn test3_3() {
        run3_3();
    }
    #[test]
    fn test3_5() {
        run3_5();
    }
}
fn run3_1() {
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);
    const THREE_HOURS_IN_SECONDS:u32 = 60*60*3;
    println!("The value of const is : {}", THREE_HOURS_IN_SECONDS);

    let x = 9;
    {
        let x = 34;
        println!("The value of x inner is:{}", x);
    }
    println!("The value of x is :{}",x);

    //let mut space = "    ";
    //space = space.len();
}

fn run3_2() {
    let guess:u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is {}", guess);
    let x = 3.0;
    let y:f32 = 4.0;
    let _z = x * y;
    let _w = y % 3.0;
    let _a = [3;5];

    let a = [2,5,7,1,9];
    let mut  index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read index");
    let index:usize = index.trim().parse().expect("Index is not a number");
    let element = a[index];
    println!("The value at index {} is {}", index, element);
}
fn run3_3() {
    fn plus_on (x:i32) -> i32 {
        x+1
    }
    let x = plus_on(3);
    println!("plus one : {}", x);
}

fn run3_5() {
    let x = 3;
    if x != 0 {
        println!("x is number")
    }
    let a = [3,6,2,1,8,6];
    for i in a {
        println!("ele is {}",i);
    }
}

