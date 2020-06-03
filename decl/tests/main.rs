use concat_strs::concat_strs;

#[test]
fn simple() {
    let s = concat_strs!["abc", "def", 'c', "h"];
    assert_eq!("abcdefch", &s);
}
