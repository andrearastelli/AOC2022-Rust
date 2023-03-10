fn main() {
    let content = std::fs::read_to_string("./data011.txt").expect("File not found");
    let splitted_data: Vec<&str> = content.split("\n").collect();
    for line in splitted_data {
        println!("Name: {line}");
    }
}
