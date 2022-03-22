pub fn clear_sreen() {
    println!("\x1Bc")
}

pub fn h_black() -> String {
    String::from("\x1b[0;90m")
}

pub fn reset() -> String {
    String::from("\x1b[0m")
}

pub fn styled() -> String {
    String::from("\x1b[1;3;4m")
}

pub fn turn(t: &char) -> String {
    format!("{}{}{}", styled(), t, reset())
}

pub fn border(t: &str) -> String {
    format!("{}{}{}", h_black(), t, reset())
}
pub fn nimp(t: &char) -> String {
    format!("{}{}{}", h_black(), t, reset())
}

pub fn num(t: &usize) -> String {
    format!("{}{}{}", h_black(), t, reset())
}
