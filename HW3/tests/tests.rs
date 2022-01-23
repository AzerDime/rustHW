use kwindex::*;

#[test]
fn test_kwindex() {
    let kwindex = KWIndex::new().extend_from_text("I am THE WALRUS");
    assert_eq!(1, kwindex.count_matches("THE"));
    assert_eq!(4, kwindex.len());
    assert_eq!(Some("WALRUS"), kwindex.nth_uppercase(2));
}
