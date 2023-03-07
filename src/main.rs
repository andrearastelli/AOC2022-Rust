// DAY 1

fn main() {
    let source_values = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    let values: Vec<&str> = source_values.split("\n").collect();

    // let mut results = Vec::new();
    let mut sum_calories_per_elf: Vec<i32> = Vec::new();
    let mut guard: bool = false;
    let mut sum_calories: i32 = 0;

    for value in values.iter() {
        if guard == true {
            guard = false;
            sum_calories_per_elf.push(sum_calories);
            sum_calories = 0;
        }

        if value.is_empty() {
            guard = true;
        } else {
            sum_calories = sum_calories + value.parse::<i32>().unwrap();
        }
    }

    let result: String = sum_calories_per_elf
        .iter()
        .map(|&id| id.to_string() + "\n")
        .collect();

    println!("Calories per elf: \n{result}");

    let max = sum_calories_per_elf.iter().max();
    match max {
        Some(max) => println!("Max: {max}"),
        None => println!("Nothing"),
    }
    // let joined_values = results.join(",");
    // println!("Values: {joined_values}");
}
