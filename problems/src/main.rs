fn ret_string() -> String {
    String::from("  A String object  ")
}
fn main() {

    // Who is the owner ?

    let s = ret_string();
    let s = s.trim();
    assert_eq!(s, "A String object");
}