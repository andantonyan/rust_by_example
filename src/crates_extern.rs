extern crate crates_extern;

fn main() {
  crates_extern::public_function();

  // Error! `private_function` is private
  //rary::private_function();

  crates_extern::indirect_access();
}
