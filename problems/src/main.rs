use std::ops::Deref;
use std::ops::DerefMut;

enum OOR<'a> {
    Owned(String),
    Borrowed(&'a str),
}

impl<'a> Deref for OOR<'a> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            OOR::Owned(s) => s.as_str(),
            OOR::Borrowed(s) => s,
        }
    }
}

impl<'a> DerefMut for OOR<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            OOR::Owned(s) => s.as_mut_str(),
            OOR::Borrowed(s) => {*self = OOR::Owned(String::from(*s)); self.deref_mut()},
        }
    }
}

fn takes_mut_str(s: &mut str) {
    println!("{} is a &mut str", s);
}

fn takes_str(s: &str) {
    println!("{} is a &str", s);
}

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

    // Write an OOR type

    let first_string = "hello".to_string();
    let second_string = "world".to_string();

    let mut p = OOR::Owned(first_string); // first_string is moved and can't be used afterwards
    let mut q = OOR::Borrowed(second_string.as_str()); // second_string is borrowed

    takes_str(&p);
    takes_str(&q);

    takes_mut_str(&mut p);
    takes_mut_str(&mut q);

    // Check Deref for both variants of OOR
    let s1 = OOR::Owned(String::from("  Hello, world.  "));
    assert_eq!(s1.trim(), "Hello, world.");
    let mut s2 = OOR::Borrowed("  Hello, world!  ");
    assert_eq!(s2.trim(), "Hello, world!");

    // Check choose
    let s = choose_str(&s1, &s2, true);
    assert_eq!(s.trim(), "Hello, world.");
    let s = choose_str(&s1, &s2, false);
    assert_eq!(s.trim(), "Hello, world!");

    // Check DerefMut, a borrowed string should become owned
    assert!(matches!(s1, OOR::Owned(_)));
    assert!(matches!(s2, OOR::Borrowed(_)));
    unsafe {
        for c in s2.as_bytes_mut() {
            if *c == b'!' {
                *c = b'?';
            }
        }
    }
    assert!(matches!(s2, OOR::Owned(_)));
    assert_eq!(s2.trim(), "Hello, world?");


}