// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    Move{x: u32, y: u32},
    Echo(String),
    ChangeColor(u32, u32, u32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }

    let lel = disemvowel("This website is for losers LOL!");
    print!("{}", lel);
}

fn disemvowel(s: &str) -> String {
    let vowels = vec!['a','e','i','o','u','A','E','I','O','U'];
    s.chars().filter(|c| !vowels.contains(c)).collect()
}
