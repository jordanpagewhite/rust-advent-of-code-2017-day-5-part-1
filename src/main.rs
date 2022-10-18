use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// data.txt is the Advent of Code 2017 Day 5 problem data.
// example.txt is the example they provide in the instructions.
const FILEPATH: &str = "/data/data.txt";

fn main() {
    let filepath = String::from(FILEPATH);
    let mut file = get_file_from_path(filepath);
    let data_str = get_data_from_file(&mut file);
    let mut data_vec = transform_data(data_str);
    let steps = step_game(&mut data_vec);
    println!("Jumped out of the maze in {} steps!", steps);
}

/// The Advent of Code 2017 Day 5 step game.
///
/// See: https://adventofcode.com/2017/day/5
///
/// # Arguments
///
/// * `vec` - (&mut Vec<i32>) A vector of integers used as the "maze".
fn step_game(vec: &mut Vec<i32>) -> i32 {
    let mut steps = 0;
    let mut index = 0;
    let mut index_previous;
    println!("----- Step {} -----", steps);

    loop {
        // If index is out of bounds, return the number of steps we've taken.
        if index >= vec.len()  {
            //println!("{:?}", &*vec);
            return steps;
        } else {
            //println!("{:?}", &*vec);
            println!("index: {}, value: {}", index, vec[index]);
            index_previous = index;
            // I am a n00b and couldn't think of a better way to do this at this point in my brand new Rust journey.
            if vec[index].is_negative() {
                index -= vec[index].unsigned_abs() as usize;
            } else {
                index += vec[index] as usize;
            }
            vec[index_previous] += 1;
            steps += 1;
            println!("----- Step {} -----", steps);
        }
    }
}

/// Loads and returns a File from its relative path.
///
/// # Arguments
///
/// * `relative_path` - (String) A relative path to the file.
fn get_file_from_path(relative_path: String) -> File {
    let basepath = env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR is not set");
    let filepath_str = format!("{}{}", basepath, relative_path);
    let filepath = Path::new(&filepath_str);
    File::open(filepath).expect("Can't open file!")
}

/// Reads and returns the contents of a file as a String.
///
/// # Arguments
///
/// * `file` - (&mut File) A file to read string data from.
fn get_data_from_file(file: &mut File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read to line.");
    contents
}

/// Transforms string data into a Vec<i32> where each line becomes an element.
///
/// # Arguments
///
/// * `data_str` - (String) A string to convert into a Vec<i32>.
fn transform_data(data_str: String) -> Vec<i32> {
    let mut vec_ints = Vec::<i32>::new();
    for line in data_str.lines() {
        vec_ints.push(line.parse::<i32>().unwrap());
    }

    vec_ints
}

/*
fn get_data_from_file() -> Vec<i32> {
    //let basepath = env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR is not set");
    //let relative = "/data/data.txt";
    //let filepath_str = format!("{}{}", basepath, relative);
    //let filepath = Path::new(&filepath_str);
    //let mut file = File::open(filepath).expect("Can't open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read to line.");

    let mut vec_ints = Vec::<i32>::new();
    for line in contents.lines() {
        vec_ints.push(line.parse::<i32>().unwrap());
    }

    vec_ints
}
 */