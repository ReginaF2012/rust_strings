fn main() {
    // create a new empty String
    let mut s = String::new();

    // another way to create a String
    let s2 = "initial contents".to_string();

    // another...
    let s3 = String::from("initial contents");

    // Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // pushing onto a string
    let mut s = String::from("foo");
    s.push_str("bar");

    // since we are using string slices (line 31) we are able to reference them again, s1 doesn't take ownership of s2
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // the push method takes a single character
    let mut s = String::from("lo");
    s.push('l');

    // + Operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //? First, s2 has an &, meaning that we’re adding a reference of the second string to the first string because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together. But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add. So why does it compile?
    
    // The answer is because a String can be coerced into a str


    // concatnating strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // The version of the code using format! is much easier to read and doesn’t take ownership of any of its parameters.

    // iterate over characters of string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // iterate over bytes in string
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // to get graphemes of strings you will have to get a crate from crates.io, because it is complex and not included in Rust's standard library.

    // strings are very complex in Rust and we are not able to easily grab a character at a specific index.


}
