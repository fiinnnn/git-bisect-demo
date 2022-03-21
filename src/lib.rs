fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(4, 6), 10);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(7, 3), 4);
    }
}
