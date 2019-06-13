extern crate proto;
extern crate protobuf;

pub fn utils_example() -> String {
    let mut x = proto::Example::new();
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

    #[test]
    fn proto_to_string() {
        // let mut x = proto::Example::new();
        // x.set_string_field("hello world".into());
        // assert_eq!(
        //     protobuf::text_proto::print_to_string(&x),
        //     r#"string_field: "hello world""#)
    }
}
