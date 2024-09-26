#[macro_export]
macro_rules! s {
    ($x:expr) => {
        String::from($x)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(String::from("foo"), s!("foo"));
    }
}
