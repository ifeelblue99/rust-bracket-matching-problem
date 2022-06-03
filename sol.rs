enum Brackets {
  Opening,
  Closing
}
fn validate_brackets(data: &str) -> Result<bool, &str> {

  let data_as_bytes = data.bytes();
  let mut count: i32 = 0;
  
  for elem in data_as_bytes {
    match elem {
      b'(' => count += 1,
      _ => count -= 1
    }
    if count < 0 {
      return Ok(false)
    } 
  }
  Ok(count == 0)
}

fn main() {
  let brackets = String::from("()()())");

  if let Ok(res) = validate_brackets(&brackets) {
    match res {
      true =>  println!("{} is valid.", brackets),
      _ => println!("{} is not valid.", brackets)
    } 
  }
}
