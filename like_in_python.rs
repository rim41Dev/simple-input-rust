fn input2(text: &str, err: &str) -> String {
    let mut var = String::new();
    println!("{}", text);
    std::io::stdin()
            .read_line(&mut var)
            .expect(err);
    var
}

fn main() {
    let some_text: String = input("Write some text: ", "Error");
    println!("{}", some_text)
}
