fn fib(n: i32) -> i32 {
  if n == 0 || n == 1 {
    1
  }
  else if n > 2^30 {
    println!("overflow");
    0
  }
  else {
    //let n1 = fib(n-1);
    //let n2 = fib(n-2);
    //n1 + n2;
    fib(n-1) + fib(n-2)
  }
}

fn main() {
  let val = 10;
  println!("fib of {0} is {1:?}", val, fib(val));
}

// rust comment
