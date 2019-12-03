use std::fs;
use std::convert::TryInto;

// Take a &str in the from i0,i1,...,in and put the i's
// into a Vec
// &str &mut Vec -> ()
fn unpack_opcodes(raw_text:&str, program:&mut Vec<i32>) -> () {
	for c in raw_text.split_terminator(',') {
		match c.parse::<i32>() {
			Ok(i) => program.push(i),
			Err(_) => ()
		}
	}

	()
}

// Based on the description on advent of code page
fn pre_modify_program(program:&mut Vec<i32>) -> () {
	program[1] = 12;
	program[2] = 2;
}


// Runs the program until halt
fn run_program(program:&mut Vec<i32>) -> i32 {
	let mut i = 0;
	let length = program.len();

	while i < length {
		let index_a:usize = program[i+1].try_into().unwrap();
		let index_b:usize = program[i+2].try_into().unwrap();
		let index_store:usize = program[i+3].try_into().unwrap();

		match program[i] {
			1 => program[index_store] = program[index_a] + program[index_b],
			2 => program[index_store] = program[index_a] * program[index_b],
			99 => break,
			_ => { println!("Unkown Opcode. Exiting"); break;}
		}

		i = i + 4;
	}

	program[0]
}

// Brute forces the program to find the correct input
fn find_input_output(program:&Vec<i32>) -> (i32,i32) {
	let (mut noun, mut verb) = (0,0);
	for i in 0..100 {
		for j in 0..100 {
			let mut program_copy = program.clone();
			program_copy[1] = i;
			program_copy[2] = j;
			if run_program(&mut program_copy) == 19690720 {
				noun = i;
				verb = j;
				break;
			}
		}
	}

	(noun,verb)
}


// Solution to day 2 of 2019 advent of code
fn main() {

	let contents = fs::read_to_string("input.txt")
		.expect("Something went wrong reading the file");

	let mut program:Vec<i32> = Vec::new();
	unpack_opcodes(&contents[..], &mut program);
	pre_modify_program(&mut program);
	println!("Value at position 0: {}", run_program(&mut program.clone()));
	let (noun, verb) = find_input_output(&program);
	println!("Noun Verb: {}", 100 * noun + verb);

}
