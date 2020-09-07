extern crate reqwest;

use std::fs::File;

pub fn download(file_name: &str) -> bool {
    let _resp = reqwest::blocking::get("https://picsum.photos/200/300");
    match _resp {
        Ok(mut res) => {
            let mut _file = File::create(file_name);
            let mut _file = _file.unwrap();
            let write_result = ::std::io::copy(&mut res, &mut _file);
            match write_result {
                Ok(_res) => true,
                Err(_e) => false,
            }
        }
        Err(e) => {
            eprintln!("error.. {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::download;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
