use death::cli;
use death::user::User;

fn predict(user: &User) {
    println!("DATE OF DEATH");
    println!("{}", user.get_death_date());
    println!("Be aware of: {}", user.get_death_reason());
}

fn main() {
    let args = cli::parse();

    let mut user = User::from(&args);

    if args.name == None {
        let name = cli::ask_name();
        user.set_id(User::get_id_from_string(&name));
    }

    if args.birthday == None {
        let age = cli::ask_birthday();
        user.set_age(age);
    }

    println!("");

    predict(&user);
}
