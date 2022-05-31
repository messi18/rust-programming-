use std::thread;
use std::time::Duration;

fn calculation_intensity(intensity:i32) -> i32 {
    println!("calculating ....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn workout(intensity:i32) {
    let cal = |i:i32| {
        thread::sleep(Duration::from_secs(2));
        i
    };
    let mut cacher = Cacher::new(cal);
    cacher.value(22);
    cacher.value(22);

}
struct Cacher<T>
where T : Fn(i32) ->i32
{
    value: Option<i32>,
    f: T
}

impl <T> Cacher<T>
    where T : Fn(i32)-> i32
{
    fn new(f: T) -> Cacher<T> {
        Cacher { f, value: None }
    }
    fn value(&mut self, i:i32) -> i32 {
        match self.value {
            Some(v) => v,
            None  =>  {
                let x = (self.f)(i);
                self.value = Some(x);
                x
            }
        }
    }
}

mod tests {
    use crate::ch13::calculation_intensity;
    #[test]
    fn test_calculation_intensity() {
        assert_eq!(12,calculation_intensity(12));
    }
}