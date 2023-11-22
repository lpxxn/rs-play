mod cat;

use cat::cat_info::Cat;

mod user_info;

use user_info::users::user;


fn main() {
    let cat = Cat::new("Mimi".to_string(), 2);
    cat.print_info();

    let user = user::User::new("Tom".to_string(), 18);
    println!("name: {}", user.name());
}

