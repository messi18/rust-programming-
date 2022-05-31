use std::fmt::{Display, Formatter};

#[cfg(test)]
mod test {
    use crate::ch10::{Point, test_10, test_iter_impl};

    #[test]
    fn test_test_10() {
        test_10();
    }
    #[test]
    fn test_point() {
        let p = Point{ x:10, y:19};
        println!("px:{}", p.x());
        let f:Point<f32,f32> = Point{x:4.0, y:3.0};
        println!("distance:{}", f.distance_from_origin());
        let p1 = Point{ x : 1, y:2};
        let p2 = Point{x:3, y:4};
        println!("p + p:{}", p1 + p2);
    }
    #[test]
    fn test_iter() {
        test_iter_impl();
    }
}
fn largest<T: PartialOrd + Copy >(list: &[T]) -> T {
    let mut la = list[0];
    for &number in list {
        if number > la {
            la = number;
        }
    }
    return la;
}
fn test_10() {
    let number_list = vec![34,50,25];
    println!("The largest number is {}", largest(&number_list));
}

struct Point<X1,Y1> {
    x:X1,
    y:Y1
}

impl <X1 ,Y1> Point<X1,Y1> {
    fn x(&self)->&X1 {
        &self.x
    }
}

impl std::ops::Add for Point<i32,i32>{
    type Output = Point<i32,i32>;

    fn add(self, other:Point<i32,i32>)-> Self {
          Point{
              x:self.x + other.x,
              y:self.y + other.y
          }
    }

}
use std::fmt;
impl Display for Point<i32,i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x:{},y:{}", self.x, self.y)
    }
}
impl Point<f32,f32> {
    fn distance_from_origin(&self)-> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}
fn test_iter_impl() {
    let points = vec![Point{x:2,y:3}, Point{x:4,y:6}];
    let x = points.iter()
        .filter(|i| i.x % 2 == 0)
        .map(ToString::to_string)
        .collect::<String>();
    println!("{}", x);
}