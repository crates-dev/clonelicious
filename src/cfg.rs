#[test]
fn test_clone() {
    use crate::*;
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    clone!(s1, s2; move |x: String, y: String| {
        assert_eq!(x, String::from("Hello"));
        assert_eq!(y, String::from("World"));
    });
}
