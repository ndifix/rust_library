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

#[test]
fn add_psmall_psmall() {
    let lhs = Bigint::from_int(100_000);
    let rhs = Bigint::from_int(200_000);
    let result = lhs + rhs;
    assert_eq!(result.to_string(), "300000");
}

#[test]
#[should_panic]
fn add_nsmall_psmall() {
    let lhs = Bigint::from_int(-100_000);
    let rhs = Bigint::from_int(200_000);
    let result = lhs + rhs;
    assert_eq!(result.to_string(), "300000");
}

#[test]
#[should_panic]
fn add_psmall_nsmall() {
    let lhs = Bigint::from_int(100_000);
    let rhs = Bigint::from_int(-200_000);
    let result = lhs + rhs;
    assert_eq!(result.to_string(), "300000");
}

#[test]
fn add_nsmall_nsmall() {
    let lhs = Bigint::from_int(-100_000);
    let rhs = Bigint::from_int(-200_000);
    let result = lhs + rhs;
    assert_eq!(result.to_string(), "-300000");
}
