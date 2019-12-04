use std::fs;

fn parse_directions(s:&str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let contents = fs::read_to_string(s).expect("Something went wrong reading the file");
    let mut a:Vec<(i32, i32)> = Vec::new();
    let mut b:Vec<(i32, i32)> = Vec::new();
    let mut i = 0;
    let (mut x, mut y);

    a.push((0, 0));
    b.push((0, 0));

    for wire in contents.split_terminator('\n') {
        let mut vec_ref = &mut a;

        if i == 0 {
            i = 1;
        } else if i == 1 {
            vec_ref = &mut b;
        }

        x = 0;
        y = 0;

        for direction in wire.split_terminator(',') {
            match (&direction[0..1], &direction[1..].parse::<i32>()) {
                ("U", Ok(length)) => y = y + length,
                ("D", Ok(length)) => y = y - length,
                ("L", Ok(length)) => x = x - length,
                ("R", Ok(length)) => x = x + length,
                (_, _) => {println!("Malformed input file statement: {}", direction); panic!();}
            }

            vec_ref.push((x, y));
        }
    }

    (a, b)
}

fn is_horizontal( ((c1_x, _), (c2_x, _)) : ((i32, i32), (i32, i32)) ) -> bool {
   c1_x == c2_x
}

fn determine_crossings(a:&Vec<(i32, i32)>, b:&Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut a_cp = a.clone();
    let mut crossings:Vec<(i32, i32)> = Vec::new();
    let mut line_a = (0, a_cp.pop());

    while !a_cp.is_empty() {
        let mut b_cp = b.clone();
        let mut line_b = (0, b_cp.pop());

        line_a = (line_a.1, a_cp.pop());

        while !b_cop.is_empty() {
            line_b = (line_b.1, b_cp.pop());

            if is_horizontal(line_a) {
                if is_horizontal(line_b) {
                    if ((line_a.0).1 == (line_b.0).1) {
                        // Add all overlapping points
                    }
                } else {
                    if ((line_a.0).0 >= (line_b.0).0 >= (line_a.1).0
                        || (line_a.0).0 <= (line_b.0).0 <= (line_a.1).0) {
                        crossings.push(((line_b.0.)0, (line_a.0).1));
                    }
                }
            } else {
                if is_horizontal(line_b) {
                    if ((line_b.0).0 >= (line_a.0).0 >= (line_b.1).0
                        || (line_b.0).0 <= (line_a.0).0 <= (line_b.1).0) {
                        crossings.push(((line_a.0).0, (line_b.0).1));
                    }
                } else {
                    if ((line_a.0).0 == (line_b.0).0) {
                        // Add all overlapping points
                    }
                }
            }

        }
    }


    Vec::new()
}

fn main() {

    let (a, b) = parse_directions("input.txt");
    detremine_crossings(&a, &b);
}
