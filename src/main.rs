use json;
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
    let body_html = message.body_html(0).unwrap();

    let mut json_obj = json::JsonValue::new_object();

    json_obj["to"] = to.into();
    json_obj["from"] = from.into();
    json_obj["subject"] = subject.into();
    json_obj["attachment_count"] = attachment_count.into();
    json_obj["body"] = body.to_string().into();
    json_obj["body_html"] = body_html.to_string().into();

    println!("{:#}", json_obj);
}
