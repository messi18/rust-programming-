mod tests {
    use crate::ch8::{use_string, use_vec};

    #[test]
   fn test_use_vec() {
        use_vec();
   }
    #[test]
    fn test_use_string() {
        use_string();
    }
}

fn use_vec() {
    let _m = vec![1, 2, 3, 4, 5];
    let mut _n = vec![0,2,4];
    let _y = &_n[0];
    // _n.push(1);
    println!("_y:{}", _y);
    let _i = &_m[0];
    let _a: i32 = match _m.get(5) {
        Some(&a) => a,
        None => -1,
    };
    println!("_a:{}, _i:{}", _a, _i);
}

fn use_string() {
    let data = "initial contents";
    println!("{}",data);
    let s = data.to_string();
    println!("{}",s);
// the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}",s + "a");
}