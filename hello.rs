fn fib(n: i32) {
  if n == 0 && n == 1 {
    1;
  }
  else {
    fib(n-1) + fib(n-2);
  }
}

fn main() {
  let val = 100;
  println!("fib of {0} is {1:?}", val, fib(val));
}

// rust comment
