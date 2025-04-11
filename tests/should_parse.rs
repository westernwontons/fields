#![allow(unused)]

use fields::Fields;

#[test]
fn should_parse_struct() {
    #[derive(Fields)]
    struct Test {
        pub field1: String,
        pub field2: String,
    }

    assert_eq!(Test::fields(), ["field1", "field2"]);
}
