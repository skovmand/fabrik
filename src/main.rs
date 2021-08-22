fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_calculate() {
        let value = 4;
        assert_eq!(value, 4);
    }
}
