// Booleans (`bool`)

fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // TODO: Define a boolean variable with the name `is_evening` before the `if` statement below.
    // The value of the variable should be the negation (opposite) of `is_morning`.
    // let …
    let is_evening = false;
    if is_evening {
        println!("Good evening!");
    }
}


struct User {
    active: bool,
    username: String,
    email: String,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
    }
}

