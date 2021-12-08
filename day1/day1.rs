use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
  let stdin = io::stdin();
  let mut lines_iter = stdin.lock().lines();

  let mut count = 0;
  let mut prev = match lines_iter.next() {
    Some(Ok(line)) => line.parse::<i32>().unwrap(),
    Some(Err(err)) => panic!("{}", err),
    None => 0
  };

  for line in lines_iter {
    let n = match line {
      Ok(s) => s.parse::<i32>().unwrap(),
      Err(err) => panic!("{}", err)
    };
    if n > prev {
      count = count + 1;
    }
    prev = n;
  }

  println!("{}", count);

  Ok(())
}
