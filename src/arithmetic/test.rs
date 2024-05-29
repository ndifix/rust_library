use super::*;

#[test]
fn fromint_positive_one_digit() {
    let bi = Bigint::from_int(20);
    let str = bi.to_string();
    assert_eq!(str, "20");
}

#[test]
fn fromint_negative_one_digit() {
    let bi = Bigint::from_int(-37);
    let str = bi.to_string();
    assert_eq!(str, "-37");
}
