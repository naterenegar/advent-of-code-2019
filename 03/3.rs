use std::fs;

fn read_directions(s:&str) -> (Vec<(&str, usize)>, Vec<(&str, usize)>) {
    let contents = fs::read_to_string(s).expect("Something went wrong reading the file");
    let a:Vec<(&str, usize)> = Vec::new();
    let b:Vec<(&str, usize)> = Vec::new();


    for wire in contents.split_terminator('\n') {
        for direction in wire.split_terminator(',') {
            let distance = &direction[1..];
            let length = distance.parse::<usize>();
/*            match (first char of direction, &direction[1..].parse::<usize>()) {
                ("U", Ok(length)) =>
                ("D", Ok(length)) =>
                ("L", Ok(length)) =>
                ("R", Ok(length)) =>
                (_, _) => {println!("Malformed input file statement: {}", direction); panic!();}
            }*/
        }
    }

    (a, b)
}


fn main() {

    read_directions("input.txt");

}
