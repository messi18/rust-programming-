use std::fmt::{Display, Formatter};

struct User {
    active: bool,
    name: String,
    mail: String,
    sign_in_count: u64
}
impl Display for User{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"name:{}, mail:{}, ative:{}, sign_in_count:{}",self.name,self.mail, self.active, self.sign_in_count)
    }
}
fn build_user(mail:String, username:String) -> User {
    User{
        mail,
        name: username,
        active: true,
        sign_in_count: 1
    }
}
#[cfg(test)]
mod test  {
    use crate::ch5::{build_user, User};

    #[test]
    fn test_struct() {
        let s = User{
            active: true,
            name: String::from("Lance ma"),
            mail: String::from("a@b.com"),
            sign_in_count:2
        };
        println!("struct is :{}", s);
        let s = build_user(String::from("b@cc.com"), String::from("lucky"));
        println!("struct is :{}", s);
        let t = User {
            active:false,
            sign_in_count:0,
            ..s
        };
        println!("struct t is :{}", t);
    }

}
