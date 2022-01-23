use kwindex::*;

#[test]
fn test_kwindex() {
    let kwindex = KWIndex::new().extend_from_text("hello world.");
    assert_eq!(1, kwindex.count_matches("hello"));
    assert_eq!(2, kwindex.len());
}