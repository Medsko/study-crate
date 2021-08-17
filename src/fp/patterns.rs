
fn destructuring_function(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn be_unintuitive() {
        for (value, index) in vec!['a', 'b', 'c'].iter().enumerate() {
            println!("Value {} is at index {} (not really, but you probably see what's going on here)",
                     value, index);
        }
    }

    #[test]
    fn destructure_tuple() {
        let tuple = (12, 382, 298);
        let (de, structured, _) = tuple;

        println!("First destructured value: {}, second: {}", de, structured);
    }

    #[test]
    fn call_destructuring_function() {
        let tuple = (32, 938);
        destructuring_function(&tuple);
    }
}