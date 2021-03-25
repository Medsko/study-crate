#![allow(dead_code)]
// I've
// had
// the lifetime of my liiife

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// pub fn invalid() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//         //  ^^ borrowed value does not live long enough
//     }
//     println!("{}", r)
// }

pub fn live_time() {
    let string1 = "abcd".to_string();
    let string2 = "xyz";

    let longest = longest(string1.as_str(), string2);
    println!("Longest string: {}", longest);

    let novel = "Call me Ishmael. Some years ago...".to_string();
    {
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("First sentence of novel: {}", i.part);
    }


}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
