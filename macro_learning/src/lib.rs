pub trait Hello {
    fn hello() -> String;
}

#[cfg(test)]
mod tests {
    use crate::Hello;
    use macro_learning_derive::HelloMacro;
    #[derive(HelloMacro)]
    struct Something;

    #[test]
    fn test_hello_macro() {
        assert_eq!(Something::hello(), "Hello, Macro! My name is Something");
    }
}
