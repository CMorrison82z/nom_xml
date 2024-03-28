use nom::{
  error::{ErrorKind},
};
use tiled_parse::*;

// fn main() {
//     let ctx = |s: &Path| PathBuf::new();
//
//     let f = File::open("t.tmx").unwrap();
//     let mut reader = BufReader::new(f);
//
//     preview(reader);
// }

fn main() {
    let blarg = std::fs::read("t.tmx").unwrap();
    let data: &str = std::str::from_utf8(&blarg).unwrap();

  println!(
    "will try to parse valid XML data:\n\n**********\n{:#?}\n**********\n",
    data
  );

  println!(
    "parsing a valid file:\n{:#?}\n",
    root::<(&str, ErrorKind)>(data)
  );
}
