use std::fmt::{ Formatter, Display, Result }; //TODO: why in the example self is imported?

struct Structure(pub i32);

struct City {
  pub name: &'static str,
  pub lat: f32,
  pub lon: f32,
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

struct List(Vec<i32>);

impl Display for List {
  fn fmt(&self, f: &mut Formatter) -> Result {
    // Dereference `self` and create a reference to `vec`
    // via destructuring.
    let List(ref vec) = *self;

    try!(write!(f, "["));

    // Iterate over `vec` in `v` while enumerating the iteration
    // count in `count`.
    for (count, v) in vec.iter().enumerate() {
      // For every element except the first, add a comma
      // before calling `write!`. Use `try!` to return on errors.
      if count != 0 { try!(write!(f, ", ")); }
      try!(write!(f, "{}", v));
    }

    // Close the opened bracket and return a fmt::Result value
    write!(f, "]")
  }
}

impl Display for Structure {
  fn fmt (&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)
  }
}

impl Display for City {
  fn fmt (&self, f: &mut Formatter) -> Result {
    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
    let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

    write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
  }
}

impl Display for Color {
  fn fmt (&self, f: &mut Formatter) -> Result {
    write!(f, "({}, {}, {}) 0x{3:X}{4:X}{5:X}", self.red, self.green, self.blue, self.red,
    self.green, self.blue)
  }
}

pub fn main() {
  let structure: Structure = Structure(10);
  println!("structure: {}", structure);

  for city in [
    City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    City { name: "Oslo", lat: 59.95, lon: 10.75 },
    City { name: "Vancouver", lat: 49.25, lon: -123.1 },
  ].iter() {
    println!("{}", *city);
    //TODO: why city and color using pointer?
  }
  for color in [
    Color { red: 128, green: 255, blue: 90 },
    Color { red: 0, green: 3, blue: 254 },
    Color { red: 0, green: 0, blue: 0 },
  ].iter() {
    // Switch this to use {} once you've added an implementation
    // for fmt::Display
    println!("{}", *color)
  }

  let v = List(vec![1, 2, 3]);
  println!("{}", v);
}
