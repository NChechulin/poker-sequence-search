#[macro_use]
extern crate enum_primitive;

mod card;
mod zfinder;

use zfinder::ZFinder;

fn main() {
    let data = "0122333301222412412".as_bytes().into();
    let pattern = "122".as_bytes().into();
    let mut finder = ZFinder::new(&data, &pattern);

    let all = finder.find_all();

    match all {
        Err(e) => println!("{}", e),
        Ok(vec) => {
            println!("All occurrences:");
            for e in vec {
                print!("{} ", e);
            }
        }
    }
}
