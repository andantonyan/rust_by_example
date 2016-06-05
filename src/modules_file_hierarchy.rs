// $ tree .
// .
// |-- my
// |   |-- inaccessible.rs
// |   |-- mod.rs
// |   `-- nested.rs
// `-- split.rs

// split.rs
// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
  println!("called `function()`");
}

fn main() {
  my::function();

  function();

  my::indirect_access();

  my::nested::function();
}

// my/mod.rs
// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
  println!("called `my::function()`");
}

fn private_function() {
  println!("called `my::private_function()`");
}

pub fn indirect_access() {
  print!("called `my::indirect_access()`, that\n> ");

  private_function();
}

// my/nested.rs
pub fn function() {
  println!("called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
  println!("called `my::nested::private_function()`");
}

// my/inaccessible.rs
#[allow(dead_code)]
pub fn public_function() {
  println!("called `my::inaccessible::public_function()`");
}

