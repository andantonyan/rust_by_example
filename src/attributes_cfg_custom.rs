// attributes_cfg_custom.rs
#[cfg(some_condition)]
fn conditional_function() {
  println!("condition met!")
}

fn main() {
  conditional_function();
}

// $ rustc custom.rs && ./custom
// No such file or directory (os error 2)

// $ rustc --cfg some_condition custom.rs && ./custom
// condition met!
