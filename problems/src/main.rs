fn ret_string() -> String {
    String::from("  A String object  ")
}

fn choose_str<'a>(s1: &'a str, s2: &'a str, select_s1: bool) -> &'a str {
    if select_s1 { s1 } else { s2 }
} // the returned str will be valid as long as both inputs are valid

fn main() {

    // Who is the owner ?

    let s = ret_string();
    let s = s.trim();
    assert_eq!(s, "A String object");

    // Select between alternatives

    let s = "hello".to_string();
    {
        let t = "bye".to_string();
        let u = choose_str(&s, &t, true);
        let v = choose_str(&s, &t, false);
        println!("First string : {u} , Second string : {v}");
    }
    //println!("u : {u} , v : {v}"); doesn't compile
}