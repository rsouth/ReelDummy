extern crate reqwest;

use std::fs::File;

pub fn download(file_name: &str) -> Result<bool, String> {
    let _resp = reqwest::blocking::get("https://picsum.photos/200/300");
    match _resp {
        Ok(mut res) => match File::create(file_name) {
            Ok(mut file) => {
                if let Err(e) = ::std::io::copy(&mut res, &mut file) {
                    eprintln!("Error: {}", e);
                    Err(e.to_string())
                } else {
                    println!("OK!");
                    Ok(true)
                }
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                Err(err.to_string())
            }
        },
        Err(e) => {
            eprintln!("error.. {}", e);
            Err(e.to_string())
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
