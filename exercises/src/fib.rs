pub fn run() {
  println!("0: {}", fib(0));
  println!("1: {}", fib(1));
  println!("2: {}", fib(2));
  println!("3: {}", fib(3));
  println!("4: {}", fib(4));
  println!("5: {}", fib(5));
  println!("6: {}", fib(6));
  println!("7: {}", fib(7));

}

fn fib(n: u32) -> u32 {
  // 0, 1
  if n < 2 {
    return n;
  }
  
  // 2
  let mut prev = 1;
  let mut res = 1;

  // > 2
  for _ in 2..n {
    let second_prev = prev;
    prev = res;
    res += second_prev;
  }
  res
}