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

#[test]
fn add_pbig() {
    let bi = Bigint::from_int(2_000_000_100);
    let mut result = bi.clone();
    for _ in 0..5 {
        result = result.clone() + bi.clone();
    }
    assert_eq!(result.to_string(), "12000000600");
}

#[test]
fn sign() {
    let pos = Bigint::from_int(123_456);
    let neg = -pos;
    assert_eq!(neg.to_string(), "-123456");
}

#[test]
fn sign2() {
    let neg = Bigint::from_int(-123_456);
    let pos = -neg;
    assert_eq!(pos.to_string(), "123456");
}
