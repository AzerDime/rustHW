use kwindex::*;

#[test]
fn test_kwindex() {
    let kwindex = KWIndex::new().extend_from_text("I am THE WALRUS");
    //Checks to see if "THE" is only present once.
    assert_eq!(1, kwindex.count_matches("THE"));
    //Checks to see the length of the provided string. "I am THE WALRUS" should return 4.
    assert_eq!(4, kwindex.len());
    //Going through upper case words starting from 0 ("I" = 0, "THE" = 1, etc), where "WALRUS" should be 2.
    assert_eq!(Some("WALRUS"), kwindex.nth_uppercase(2));

    let kwindex2 = KWIndex::new().extend_from_text("THE quick BROWN fox JUMPS over THE lazy DOG.");
    //Checks to see if "THE" is present twice.
    assert_eq!(2, kwindex2.count_matches("THE"));
    //Checks to see the length of the provided string. Should return 9.
    assert_eq!(9, kwindex2.len());
    //Going through upper case words starting from 0. "JUMPS" should be 2.
    assert_eq!(Some("JUMPS"), kwindex2.nth_uppercase(2));
}
