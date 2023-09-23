pub fn greet(name: &str) -> String {
    if name.is_empty() {
        format!("Hello World !")
    } else {
        format!("Hello {} !", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = greet("ludndev");
        assert_eq!(result, "Hello ludndev !");
    }

    #[test]
    fn it_works_empty() {
        let result = greet("");
        assert_eq!(result, "Hello World !");
    }
}
