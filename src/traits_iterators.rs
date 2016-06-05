struct Fibonacci {
  curr: u32,
  next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
  type Item = u32;

  // Here, we define the sequence using `.curr` and `.next`.
  // The return type is `Option<T>`:
  //     * When the `Iterator` is finished, `None` is returned.
  //     * Otherwise, the next value is wrapped in `Some` and returned.
  fn next(&mut self) -> Option<u32> {
    let new_next = self.curr + self.next;

    self.curr = self.next;
    self.next = new_next;

    // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
    // will never return `None`, and `Some` is always returned.
    Some(self.curr)
  }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
  Fibonacci { curr: 1, next: 1 }
}

pub fn main() {
  // `0..3` is an `Iterator` that generates: 0, 1, and 2.
  let mut sequence = 0..3;

  println!("Four consecutive `next` calls on 0..3");
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());

  // `for` works through an `Iterator` until it returns `None`.
  // Each `Some` value is unwrapped and bound to a variable (here, `i`).
  println!("Iterate through 0..3 using `for`");
  for i in 0..3 {
    println!("> {}", i);
  }

  // The `take(n)` method reduces an `Iterator` to its first `n` terms.
  println!("The first four terms of the Fibonacci sequence are: ");
  for i in fibonacci().take(4) {
    println!("> {}", i);
  }

  // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
  println!("The next four terms of the Fibonacci sequence are: ");
  for i in fibonacci().skip(4).take(4) {
    println!("> {}", i);
  }

  let array = [1u32, 3, 3, 7];

  // The `iter` method produces an `Iterator` over an array/slice.
  println!("Iterate the following array {:?}", &array);
  for i in array.iter() {
    println!("> {}", i);
  }
}
