use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
// approx crate::file::function

fn load_from_file_p1(file_path: &str) {
    let mut num_increased: i32 = 0;

    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();


    let mut i = 0;
    while i < numbers.len() - 1 {
        if numbers[i] < numbers[i + 1] {
            num_increased = num_increased + 1;
        }
        i = i + 1;
    }

    println!("{}", num_increased);
}

fn load_from_file_p2(file_path: &str) {
  // this is part 2
    let mut num_increased: i32 = 0;

    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut x_vec: Vec<i64> = Vec::new();
    let mut i = 0;
    while i < numbers.len() - 2 {
        let mut x: i64 = numbers[i] + numbers[i+1] + numbers[i+2];
        /*
        println!("{}", numbers[i]);
        println!("{}", numbers[i+1]);
        println!("{}", numbers[i+2]);
        */

        x_vec.push(x);
        x = 0;
        i = i + 1;
    }

    

    let mut j = 0;
    while j < x_vec.len() - 1 {
        if x_vec[j] < x_vec[j + 1] {
            num_increased = num_increased + 1;
        }
        j = j + 1;
    }
    
    let mut k = 0;
    while k < x_vec.len() - 1 {
        println!("{}", x_vec[k]);
        k = k + 1;
    }


    println!("{}", num_increased);
    
}

fn main() {

  //load_from_file_p1("nums.txt");

  load_from_file_p2("input.txt");

}
