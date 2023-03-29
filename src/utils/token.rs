use nanoid::nanoid;

pub fn token_generator() -> String {
    nanoid!(32)
}

#[cfg(test)]
mod token_generator_test {
    use super::*;

    #[test]
    fn test_token_generator() {
        let token = token_generator();
        println!("{}", token);
        assert_eq!(token.len(), 32);
    }
}
