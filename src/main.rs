extern crate lopdf;
extern crate chrono;

use std::{env, fs, str};
use std::path::PathBuf;

use chrono::NaiveDateTime;

fn main() {
  let document = env::args().skip(1).next().expect("Wrong number of arguments");
  let d = lopdf::Document::load(document.clone()).unwrap();


  for object in d.objects.values() {
    if let lopdf::Object::Dictionary(d) = object {
      match d.get("CreationDate") {
        Some(lopdf::Object::String(bytes, _format)) => {
          let s = str::from_utf8(bytes).unwrap();
          let date = NaiveDateTime::parse_from_str(&s[2..16], "%Y%m%d%H%M%S")
            .unwrap()
            .format("%F %T");;
          let mut target = PathBuf::from(&document);
          target.set_file_name(format!("{}.pdf", date));
          fs::copy(&document, target).unwrap();
          fs::remove_file(&document).unwrap();
          
          break;
        },
        _ => ()
      };
    }
  }
}
