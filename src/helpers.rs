pub fn split_into_lines<'a>(input: &'a str) -> Vec<String> {
    input.replace("\r", "").split("\n").map(|s| s.to_string()).collect()
}