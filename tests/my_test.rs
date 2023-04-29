fn example_to_test() -> i32 {
  12
}

#[cfg(test)]
mod tests {
  // simple example to show how to use tests
  use super::example_to_test;

  #[test]
  fn verify_function_to_test() {
    assert_eq!(example_to_test(), 12)
  }
}