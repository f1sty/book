use adder;

#[test]
fn adder_test() {
    let expected = 42;
    assert_eq!(expected, adder::add(21, 21));
}
