use std::fs::File;
use std::io::prelude::*;
//use std::vec::Vec;

// https://www.youtube.com/watch?v=Mcuqzx3rBWc

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

fn f1(file_name: &str) {
  let mut file = File::open(file_name)
    .expect("Can't open file!");

  let mut contents = String::new();
  file.read_to_string(&mut contents)
    .expect("Oops! Can not read the file...");

  //println!("File Contents:\n\n{}", contents);

  let mut i: i32 = 0;
  let mut tuple_vec: Vec<(&str, &str)>;
  //let mut vec = Vec::new();
  //let mut tv: Vec<(&str, &str)> = vec![];
  //let mut tuple = ();
  let mut horizontal_pos: i32 = 0;
  let mut depth: i32 = 0;

  for line in contents.lines() {
    
    //println!("{}", line);
    let (s1, s2) = line.split_once(' ').unwrap();

    let num = s2.parse::<i32>().unwrap();

    match s1 {
      "forward" => horizontal_pos += num,
      "down" => depth += num,
      "up" => depth -= num,
      _ => println!("one of the commands was incorrect"),

    }

    
    //tuple_vec[i] = ((s1, s2));
    //print_type_of(&s2);
    //println!("{} {}", s1, s2);
    
    
    
    /*
    if i % 2 == 0 {
      println!("{}", line);
    }
    */
    i += 1;

  }
  println!("{} {}", horizontal_pos, depth);
  

}

fn main() {

  f1("input.txt");

}
