pub fn init() {
    println!("init author lib.");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        init();
    }
}
