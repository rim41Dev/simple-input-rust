fn line_read(var: &mut String, err: &str) {
    std::io::stdin().read_line(var).expect(err);
    ()
}

fn main() {
    let mut some_var = String::new();
    println!("Write some text:");
    line_read(&mut some_var, "Error text");
    println!("You wrote {}", some_var);
}
