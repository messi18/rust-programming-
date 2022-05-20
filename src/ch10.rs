mod test {
    use crate::ch10::test_10;

    #[test]
    fn test_test_10() {
        test_10();
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