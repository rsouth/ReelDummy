extern crate reqwest;

use std::fs::File;

pub fn download(width: i32, height: i32, file_name: &str) -> Result<bool, String> {
    let url = format!("{}/{}", width, height);
    let resp = reqwest::blocking::get(&format!("https://picsum.photos/{}", url));
    match resp {
        Ok(mut res) => match File::create(file_name) {
            Ok(mut file) => save_file(&mut res, &mut file),
            Err(err) => Err(err.to_string()),
        },
        Err(e) => {
            eprintln!("error.. {}", e);
            Err(e.to_string())
        }
    }
}

fn save_file(res: &mut reqwest::blocking::Response, file: &mut File) -> Result<bool, String> {
    if let Err(e) = ::std::io::copy(res, file) {
        eprintln!("Error: {}", e);
        Err(e.to_string())
    } else {
        println!("OK!");
        Ok(true)
    }
}

// #[cfg(test)]
// use mockall::{automock, mock, predicate::*};
// use reqwest::blocking::Response;
//
// #[cfg_attr(test, automock)]
// trait MyTrait {
//     fn foo(&self, x: u32) -> u32;
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // use crate::download;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
//
//     #[test]
//     fn mytest() {
//         let mut mock = MockMyTrait::new();
//         mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
//         assert_eq!(5, mock.foo(4));
//     }
// }
