use std::io::Read;

pub fn load_inputs(day: usize) -> Vec<String> {
    let mut file = std::fs::File::open(format!("inputs/{}", day)).expect("Can't open input file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("Can't read input file");
    input.lines().map(|line| line.to_string()).collect()
}