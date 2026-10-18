// fn main() {
//     println!("Hello, world!");
//     println!("Environment testing-------");
//     println!("Environment working fine");
// }

//exploring functions in rust
fn join_call(name: &str) {
    println!("{} joined the call.", name);
}

fn leave_call(name: &str) {
    println!("{} left the call.", name);
}

fn main() {
    println!("Welcome to Rust Video Caller!");

    join_call("User 1");
    join_call("User 2");

    println!("Call connected!");

    leave_call("User 1");
    leave_call("User 2");

    println!("Call ended.");
}
