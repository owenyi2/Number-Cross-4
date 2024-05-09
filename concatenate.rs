use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;

fn read_rows() -> HashMap<usize, Vec<String>> {
    let contents = fs::read_to_string("results.txt").unwrap();
    let mut rows = HashMap::new();
    let mut row = 0;

    for line in contents.lines() {
        match &line.trim().parse::<usize>() {
            Ok(num) => {
                row = *num;
            }
            Err(_) => {
                rows.entry(row)
                    .and_modify(|row: &mut Vec<String>| row.push(line.trim().replace("\"", "")))
                    .or_insert(vec![line.trim().replace("\"", "")]);
            }
        }
    }
    rows
}

const PUZZLE_SIZE: usize = 11;

#[derive(Debug)]
enum RegionConstraint {
    Same,
    Different,
}

type RegionConstraintArray = [RegionConstraint; PUZZLE_SIZE];

fn region_to_constraint(region: &str) -> Result<RegionConstraintArray, ()> {
    let region_vec: Vec<RegionConstraint> = region
        .chars()
        .map(|c| match c {
            'S' => Ok(RegionConstraint::Same),
            'D' => Ok(RegionConstraint::Different),
            _ => Err(()),
        })
        .collect::<Result<Vec<_>, ()>>()?;
    Ok(region_vec.try_into().map_err(|_| eprintln!("Wrong size"))?)
}

fn check_constraint(
    first_row: &[char],
    second_row: &[char],
    region_constraint: &RegionConstraintArray,
) -> bool {
    for i in 0..11 {
        if first_row[i] == '*' {
            if second_row[i] == '*' {
                return false;
            }
            continue;
        }
        match region_constraint[i] {
            RegionConstraint::Same => {
                if !(second_row[i] == first_row[i] || second_row[i] == '*') {
                    return false;
                }
            }
            RegionConstraint::Different => {
                if second_row[i] == first_row[i] {
                    return false;
                }
            }
        }
    }
    return true;
}

struct Solver {
    solution: Vec<usize>,
    solutions: Vec<Vec<usize>>,
    region_constraints: Vec<RegionConstraintArray>,
    candidate_rows: HashMap<usize, Vec<String>>,
}

impl Solver {
    fn new(
        region_constraints: Vec<RegionConstraintArray>,
        candidate_rows: HashMap<usize, Vec<String>>,
    ) -> Solver {
        Solver {
            solution: vec![0], 
            solutions: Vec::new(),
            region_constraints,
            candidate_rows,
        }
    }
    fn solver(&mut self) {
        let mut level = self.solution.len();
        while !self.solution.is_empty() {
            'inner: while self.check_latest() {
                level = self.solution.len();
                if level == PUZZLE_SIZE {
                    self.solutions.push(self.solution.clone()); 
                }
                self.solution.push(0);
                dbg!(&self.solution);
                break 'inner
            }
            level = self.solution.len();
            let mut latest = self.solution.pop().expect("while !self.solution.is_empty()"); 
            latest += 1;

            if let Some(row) = self.candidate_rows.get(&(level - 1)) {
                if latest < row.len() {
                    self.solution.push(latest);
                }
            }        
        }
    }

    fn check_latest(&self) -> bool {
        let solution_length = self.solution.len();
        if solution_length <= 1 {
            return true;
        } else {
            let last = self.solution[solution_length - 1];
            let second_last = self.solution[solution_length - 2];

            let char_arr_1: Vec<char> = self.candidate_rows.get(&(solution_length - 2)).unwrap()
                [second_last]
                .chars()
                .collect();
            let char_arr_2: Vec<char> = self.candidate_rows.get(&(solution_length - 1)).unwrap()[last]
                .chars()
                .collect();

            // dbg!(&char_arr_1, &char_arr_2, &self.region_constraints[solution_length - 2]);

            check_constraint(
                &char_arr_1,
                &char_arr_2,
                &self.region_constraints[solution_length - 1 - 1],
                // -1 for getting last element of array
                // -1 for region_constraints is of length PUZZLE_LENGTH - 1, with the first element being for the second row
            )
        }
    }
}

fn main() {
    let region_constraints = vec![
        "SDDDSSSDSSD", // 2nd row
        "SSSDSSSSSSS", // 3rd row
        "SSSSSDDSDDS", // 4th row
        "SSDSDDSSSDS", // 5th row
    ];
    let rows = read_rows();

    // let mut solver = Solver::new(
    //     region_constraints
    //         .into_iter()
    //         .map(|region| region_to_constraint(region).unwrap())
    //         .collect(),
    //     rows,
    // )
    // solver.solver();

    for (idx, region) in region_constraints.into_iter().enumerate() {
        let region_constraint = region_to_constraint(&region).unwrap();
        let first = rows.get(&idx).unwrap();
        let second = rows.get(&(idx+1)).unwrap();
        for f in first {
            let f_char_arr: Vec<char> = f.chars().collect();
            for s in second {
                let s_char_arr: Vec<char> = s.chars().collect();
                let check = check_constraint(&f_char_arr, &s_char_arr, &region_constraint);
                if check {
                    println!("first: {:?}, second: {:?} | {:}", &f, &s, idx);
                }
            }
        }
    }
}

