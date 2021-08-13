fn main() {
   let  s = String::from("Hello, world!");
   let hello = &s[0..5];
   let world = &s[7..];
   println!("{} {}", hello, world);
   let first_word = first_word(&s);
   println!("first word of [{}] is [{}]", s, first_word);
   println!("first word of [{}] is [{}]", s, first_word_v2(&s));

   let a = [1, 2, 3];
   let b = &a[0..2];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item  == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}


fn first_word_v2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item  == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}