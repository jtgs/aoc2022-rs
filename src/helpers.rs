pub fn split_into_lines(input: &str) -> Vec<String> {
    input.replace("\r", "").split('\n').map(|s| s.to_string()).collect()
}

pub fn load_input(day: usize) -> String {
    std::fs::read_to_string(format!("{}/{}", "inputs", day))
        .expect("Can't open/read input file")
}