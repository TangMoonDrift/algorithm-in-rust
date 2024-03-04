pub fn print_binary(num: i32) {
    let mut i = 31;
    let mut result = String::new();
    while i >= 0 {
        match num & (1 << i) {
            0 => {
                result += "0";
            }
            _ => {
                result += "1";
            }
        }
        i -= 1
    }
    println!("{}", result);
}
