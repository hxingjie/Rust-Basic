// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


// 调整顺序
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
// }
