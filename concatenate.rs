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

fn main() {
    let region_constraints = vec![
        "SDDDSSSDSSD", // 2nd row
        "SSSDSSSSSSS", // 3rd row
        "SSSSSDDSDDS", // 4th row
        "SSDSDDSSSDS", // 5th row
        "SDDDSSDSDSD", 
        "DSSSSDDSSSD",
        "SDDSDSSSDSD", // 8th 
        "SSSDSSSSSDS",
        "SDSDDDSSSSD",
        "SDDSSDSDSSS", // 11th
    ];
    let rows = read_rows();

    println!("source,destination,level");

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
                    println!("{:?},{:?},{:}", &f, &s, idx);
                }
            }
        }
    }
}

