use std::{fs, io::Read};

#[cfg(test)]
mod test;

#[allow(dead_code)]
struct Pdf {}

#[allow(dead_code)]
fn get_pdf(path: &str) -> Pdf {
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("failed to open file {path}: {e}"),
    };

    let mut buf: Vec<u8> = Vec::new();
    let _length = match file.read_to_end(&mut buf) {
        Ok(len) => len,
        Err(e) => panic!("Failed to read file: {e}"),
    };

    Pdf {}
}
