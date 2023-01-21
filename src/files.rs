use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

fn main() {
    let mut file = match File::open("data/foo.csv") {
        Err(err) => panic!("Couldn't open: {}", err),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
        Err(err) => panic!("Couldn't get stat: {}", err),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
        Err(err) => panic!("Couldn't read: {}", err),
        Ok(_) => (),
    };

    let data = match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err),
        Ok(value) => value,
    };

    println!("Content is: {}", data);
}
