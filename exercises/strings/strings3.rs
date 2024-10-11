// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut i = 0;
    let mut lef = 0;
    let mut righ = 0;
    for c in input.chars() {
      if c != ' ' {righ = i + 1;}
      if c == ' ' && righ == 0 {lef = i + 1;}
      i += 1;
    }
    input[lef..righ].to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    (input.to_string() + " world!").to_string()
}

fn replace_me(input: &str) -> String {
  let mut T = String::from("");
  let Cr = ['c', 'a', 'r', 's'];
  let mut i = 0;
  let mut j = 0;
  let mut Last = 0;
  for c in input.chars() {
    if c == Cr[j] {
      j += 1;
      if j == 4 {
        T += &input[Last..i]; T += "balloons";
        i += j; j = 0; Last = i;
      }
    }
    else {i += j + 1; j = 0;}
  }
  // eprintln!("He{}, {}", Last, i);
  T += &input[Last..i];
  T
    // TODO: Replace "cars" in the string with "balloons"!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
