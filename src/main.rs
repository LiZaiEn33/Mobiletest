fn main() {
    println!("Hello, world from Rust!");
    println!("This is a test environment for Rust in Codespaces");
    
    // Test some basic operations
    let result = add_numbers(5, 3);
    println!("5 + 3 = {}", result);
    
    let greeting = greet("Codespaces");
    println!("{}", greeting);
}

// A simple function to add two numbers
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// A simple function to create a greeting
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(-1, 1), 0);
        assert_eq!(add_numbers(0, 0), 0);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
}
