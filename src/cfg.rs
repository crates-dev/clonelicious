#[test]
fn test_clone() {
    use crate::*;
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res: String = clone!(s1, s2, {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{} {}", s1, s2)
    });
    assert_eq!(res, String::from("Hello World"));
}
