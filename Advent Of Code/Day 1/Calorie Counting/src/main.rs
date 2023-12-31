use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
mod quicksort;

fn main() {
    // file path
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Attempt to open the file
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening the file: {}", error);
            return;
        }
    };

    let mut v = Vec::new();

    // Use a BufReader to read the file line by line
    let reader = BufReader::new(file);

    let mut sum = 0;

    // Process each line in the file
    for line_result in reader.lines() {
        match line_result {
            Ok(line_content) => {
                if line_content.is_empty() {
                    v.push(sum);
                    sum = 0;
                } else {
                    match line_content.parse::<i32>() { 
                        Ok(number) => { 
                            sum = sum + number;
                        }
                        Err(error) => { 
                            eprintln!("Oopsies {}\n", error);
                            return;
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
            }
        }
    }

    //Part one
    //-------------------------------------------------------------
    let mut global_max = 0;

    for sum in &v { 
        if sum > &global_max { 
            global_max = *sum;
        }
    }

    println!("{global_max}");
    //-------------------------------------------------------------
    


    //Part two
    //-------------------------------------------------------------
    quicksort::quick_sort(&mut v);

    let a = v.pop();
    let b = v.pop();
    let c = v.pop();

    let total = a.unwrap() + b.unwrap() + c.unwrap();
    println!("{total}");
    //-------------------------------------------------------------

}
