pub fn load_input(day: usize) -> String {
    std::fs::read_to_string(format!("{}/{}", "inputs", day))
        .expect("Can't open/read input file")
}