use mail_parser::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let filepath = env::args().nth(1).expect("no file given");
    let f = File::open(filepath).expect("can't read the file");

    let mut reader = BufReader::new(f);
    let mut vec = Vec::new();
    let _ = reader.read_to_end(&mut vec);

    let message = MessageParser::default().parse(&vec).unwrap();

    let to: Vec<_> = message
        .to()
        .unwrap()
        .iter()
        .map(|x| x.address().unwrap())
        .collect();
    let from: Vec<_> = message
        .from()
        .unwrap()
        .iter()
        .map(|x| x.address().unwrap())
        .collect();
    let subject = message.subject().unwrap();
    let attachment_count = message.attachment_count();
    let body = message.body_text(0).unwrap();

    println!("From: {:?}", from);
    println!("To: {:?}", to);
    println!("Subject: {:?}", subject);
    println!("Body: {:?}", body);
    println!("Attachment count: {:?}", attachment_count);
}
