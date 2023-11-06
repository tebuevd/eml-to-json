use json;
use json::object;
use json::JsonValue;
use mail_parser::HeaderValue;
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

    let attachment_count = message.attachment_count();
    let body = message.body_text(0).unwrap();
    let body_html = message.body_html(0).unwrap();

    let mut headers_arr = JsonValue::new_array();

    for header in message.headers() {
        match &header.value {
            HeaderValue::ContentType(value) => {
                let header_blob = object! {
                    name: header.name(),
                    value: object! {
                        type: value.c_type.as_ref(),
                        subtype: match value.c_subtype {
                            Some(ref subtype) => subtype.as_ref(),
                            None => ""
                        },
                    }
                };

                headers_arr.push(header_blob).unwrap();
            }
            HeaderValue::Text(value) => {
                let header_blob = object! {
                    name: header.name(),
                    value: value.as_ref()
                };

                headers_arr.push(header_blob).unwrap();
            }
            _ => {}
        }
    }

    let mut json_obj = JsonValue::new_object();

    json_obj["from"] = match message.from() {
        None => JsonValue::new_array(),
        Some(item) => item
            .iter()
            .map(|i| {
                let mut data = JsonValue::new_object();

                data["name"] = i.name().into();
                data["address"] = i.address().into();
                return data;
            })
            .collect::<Vec<_>>()
            .into(),
    };

    json_obj["to"] = match message.to() {
        None => JsonValue::new_array(),
        Some(to_fields) => to_fields
            .iter()
            .map(|address| {
                let mut data = JsonValue::new_object();

                data["name"] = address.name().into();
                data["address"] = address.address().into();

                return data;
            })
            .collect::<Vec<_>>()
            .into(),
    };

    json_obj["subject"] = match message.subject() {
        Some(subject) => subject.into(),
        None => JsonValue::Null,
    };

    json_obj["date"] = match message.date() {
        Some(date) => date.to_rfc3339().into(),
        None => JsonValue::Null,
    };

    json_obj["attachment_count"] = attachment_count.into();
    json_obj["body"] = body.to_string().into();
    json_obj["body_html"] = body_html.to_string().into();
    json_obj["raw_headers"] = headers_arr;

    println!("{:#}", json_obj);
}
