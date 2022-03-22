pub fn clear_sreen() {
    println!("\x1Bc")
}

pub fn turn(t: &char) -> String {
    format!("\x1b[1;3;4m{}\x1b[0m", t)
}
