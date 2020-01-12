#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub mod io {
    use std::str::FromStr;
    pub fn read_number<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
        let mut input = String::new();
    
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        input.trim().parse::<T>()
    }
}
