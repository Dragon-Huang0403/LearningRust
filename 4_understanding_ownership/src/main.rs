// Stack: store known, fix sized data
// Heap: store others

fn main() {
    // Literal is immutable, store in stack.
    let ss = "immutable";
    println!("{ss}");

    // String is mutable, store in heap.
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2:?}?");
    let s3 = take_ownership_back(s2);
    take_ownership(&s3);

    borrowing();

    let sss = String::from("aaaa cccc");
    let word = first_word(&sss);
    // sss.clear();
    println!("{}", word);
}

fn take_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn take_ownership_back(some_string: String) -> String {
    some_string
}

fn borrowing() {
    let mut s = String::from("borrowing");

    {
        // s2 scope only in the bracket
        let s2 = &mut s;
        println!("{s2}");
    }

    let s1 = &mut s;
    // let s2 = &mut s; // will panic, can only borrow mutable reference once at a time

    println!("{s1}"); // s1 scope ends here, end at last usage

    let s2 = &mut s;

    println!("{s2}");
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
