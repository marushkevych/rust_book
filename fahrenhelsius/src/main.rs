use rust_utils::io::read_number;

fn main() {
    println!("Input your Fahrenheits");

    loop {

        let input: i32 = match read_number() {
            Ok(num) => num,
            Err(_) => continue
        };

        let celsius = (input - 32) *5/9;
        let celsous_char = '\u{2103}';
        println!("{}{}", celsius, celsous_char);
    }
}
