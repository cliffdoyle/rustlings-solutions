// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" { "Yummy!" } else { "1"}
}

fn main() {
    // You can optionally experiment here.
    let foo=picky_eater("banaana");
    println!("foo {}", foo);
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
   use super::*;
#[test]
   fn sweet_strawberry(){
    assert_eq!(picky_eater("strawberry"),"Yummy!");
   }

   #[test]
   fn no_strawberry(){
    assert_eq!(picky_eater("xanthi"),"1")
   }
#[test]
   fn gotcha_strawy(){
    assert_eq!(picky_eater(""),"1")
   }
}
