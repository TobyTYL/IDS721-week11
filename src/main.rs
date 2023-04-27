use rand::Rng;

fn main() {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let password_length = 12;
    let password: String = (0..password_length)
        .map(|_| {
            let index = rand::thread_rng().gen_range(0..chars.len());
            chars[index] as char
        })
        .collect();

    println!("Generated password: {}", password);
}
