extern crate proto;

use proto::*;

pub fn utils_example() -> String {
    let mut x = Example::new();
    x.set_string_field("hello world".into());
    x.string_field.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn util_example() {
        assert_eq!(utils_example(), "hello world");
    }
}
