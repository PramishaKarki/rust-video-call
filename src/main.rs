// // fn main() {
// //     println!("Hello, world!");
// //     println!("Environment testing-------");
// //     println!("Environment working fine");
// // }

// //exploring functions in rust
// fn join_call(name: &str) {
//     println!("{} joined the call.", name);
// }

// fn leave_call(name: &str) {
//     println!("{} left the call.", name);
// }

// fn main() {
//     println!("Welcome to Rust Video Caller!");

//     join_call("User 1");
//     join_call("User 2");

//     println!("Call connected!");

//     leave_call("User 1");
//     leave_call("User 2");

//     println!("Call ended.");
// }



use std::io;

fn main() {
    let mut user1_joined = false;
    let mut user2_joined = false;
    let mut call_active = false;

    println!("Welcome to Rust Video Caller!");

    loop {
        println!("\n--- Call Menu ---");
        println!("1. User 1 joins");
        println!("2. User 2 joins");
        println!("3. Start call");
        println!("4. End call");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                user1_joined = true;
                println!("User 1 joined the call.");
            }

            "2" => {
                user2_joined = true;
                println!("User 2 joined the call.");
            }

            "3" => {
                if call_active {
                    println!("Call is already active.");
                } else if user1_joined && user2_joined {
                    call_active = true;
                    println!("Call connected!");
                } else {
                    println!("Both users must join first.");
                }
            }

            "4" => {
                call_active = false;
                user1_joined = false;
                user2_joined = false;
                println!("Call ended.");
            }

            "5" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid option. Try again."),
        }
    }
}

