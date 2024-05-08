use std::fs;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

const PUZZLE_SIZE: usize = 11;

#[derive(Debug)]
enum RegionConstraint {
    Same,
    Different
}

#[derive(Debug, PartialEq)]
enum TemplateConstraint {
    Empty,
    Black
}

#[derive(Debug)]
enum Constraint {
    NeqZero,
    Equal,
    Nequal,
    Black
}

type RegionConstraintArray = [RegionConstraint; PUZZLE_SIZE];
type TemplateConstraintArray = [TemplateConstraint; PUZZLE_SIZE];
type ConstraintArray = [Constraint; PUZZLE_SIZE];

fn region_to_constraint(region: &str) -> Result<RegionConstraintArray, ()> {
    let region_vec: Vec<RegionConstraint> = region.chars().map(|c| match c {
        'S' => Ok(RegionConstraint::Same),
        'D' => Ok(RegionConstraint::Different),
        _ => Err(())
    }).collect::<Result<Vec<_>, ()>>()?;
    Ok(region_vec.try_into().map_err(|_| eprintln!("Wrong size"))?)
}

fn template_to_constraint(template: &str) -> Result<TemplateConstraintArray, ()> {
    let template_vec: Vec<TemplateConstraint> = template.chars().map(|c| match c {
        '_' => Ok(TemplateConstraint::Empty),
        '*' => Ok(TemplateConstraint::Black),
        _ => Err(())
    }).collect::<Result<Vec<_>, ()>>()?;
    Ok(template_vec.try_into().map_err(|_| eprintln!("Wrong size"))?)
}

fn lex_constraints(region: &RegionConstraintArray, template: &TemplateConstraintArray) -> ConstraintArray {
    let mut constraint_vec = Vec::new();
    let mut new_region = false;

    for i in 0..11 {
        if template[i] == TemplateConstraint::Black {
            new_region = true;
            constraint_vec.push(Constraint::Black);
            continue
        }
        if i == 0 || new_region {
            constraint_vec.push(Constraint::NeqZero); 
            new_region = false;
            continue
        }
        constraint_vec.push(match region[i] {
            RegionConstraint::Same => Constraint::Equal,
            RegionConstraint::Different => Constraint::Nequal,
        });
    }   
    constraint_vec.try_into().expect("range 0..11")
}

#[derive(Debug)]
struct Solver<'ca> {
    solutions: Vec<String>,
    constraint: &'ca ConstraintArray,
    solution: String,
    checker: fn(i64) -> bool
}

impl<'ca> Solver<'ca> {
    fn new(constraint: &'ca ConstraintArray, checker: fn(i64) -> bool) -> Solver {
        Solver {
            solutions: vec![],
            constraint,
            solution: String::new(),
            checker
        } 
    }
    fn rec_solve(&mut self, constraint_index: usize, prev: char) {
        // dbg!(constraint_index);

        // TODO: check if solutions exist and terminate if not
        if self.solution.len() == PUZZLE_SIZE {
            if !self.apply_check() {
                return
            }
            self.solutions.push(self.solution.clone());
            return
        }
        let possible_set = match &self.constraint[constraint_index] {
            Constraint::NeqZero => HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9']),
            Constraint::Equal => HashSet::from([prev]),
            Constraint::Nequal => {
                let mut set = HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
                set.remove(&prev);
                set
            },
            Constraint::Black => HashSet::from(['*'])
        };

        for c in possible_set.iter() {
            if c == &'*' { 
                if !self.apply_check() {
                    return
                }
            }

            // We are checking too early before the latest number is properly formed. We need to wait until the number is terminated by an "*" or by 

            self.solution.truncate(constraint_index);
            self.solution += &c.to_string();
           
            self.rec_solve(constraint_index + 1, *c);               
        } 
    }

    fn apply_check(&self) -> bool { // true means passes check
        if let Some(num) = &self.solution
            .split("*")
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .last() { 
            if let Ok(num) = num.parse::<i64>() {
                return (self.checker)(num) 
            }
        };
        return true 
    }

    fn solve(&mut self) {
        self.rec_solve(0, '_');
    }
}

fn main() {
    let region_constraints = vec![
        "SSSDSSDSDSS",
    ];
    let contents = fs::read_to_string("templates.txt").unwrap();

    let mut solutions: HashMap<String, Vec<String>> = HashMap::new();

    for region in region_constraints {
        let region_constraint = region_to_constraint(&region).unwrap();
        for template in contents.lines() {
            let template_constraint = template_to_constraint(&template).unwrap();
            let constraint = lex_constraints(&region_constraint, &template_constraint);

            let mut solver = Solver::new(&constraint, check_square);
            solver.solve();
            solutions
                .entry(String::from(region))
                .and_modify(|solns| solns.extend_from_slice(&solver.solutions))
                .or_insert(solver.solutions);
        }
    } 
    dbg!(&solutions);
}

fn check_square(num: i64) -> bool {
    let sqrt = (num as f64).sqrt();
    sqrt == sqrt.round()
}


// e.g. the constraint will look like this: !0, =, =, !=, !0, !=, =, !=, =

// if * then *
// if after * or initial, !0
// otherwise if s then = else d then !=

/*
e.g.
_ _ _|_ _ _|_ _|_ _ _
_ _ _ _ * _ _ _ _ _ *

Therefore we require that 

0 1 2 are same
3 != 2
* 
5 != 6
6 7 are same
8 != 7
8 9 same


*/

/* 
I think we should try backtracking. 

so we need a function

trait generate_possibilities {
    fn gen_pos()
}

*/

// === // 

// In order to satisfy constraints, this is how we will do it
// The constraints in effect can all be represented as $n digit number where the first $k digits belong to the set $S. such that the number satisfies $clue
// E.g. consider the above example.
// We will first look for 4 digit number such that the first 3 digits are in the set {111, 222, 333, 444, 555, 666, 777, 888, 999}
  // Shit: there is the additional constraint that the 4th digit is not the same as the first three digits
  // In this case the constraints are a bit more annoying but can still represent it as 1112, 1113, 1114, ... , 1119, 2221, 2223, ..., 2229, ..., 9998
// Suppose we pick 4445 for the first cell. For the second cell we have the constraint that 1st digit is not equal to the 2nd digit. the 2nd digit equals to the 3rd digit the 4th digit != 3rd digit. 5th digit == 5th digit. 
// Ok this method of representing the constraints is a bit dubious because it will be another combinatorial explosion. Although probably still a rather manageable combinatorial explosion. 
// So we should try backtracking to satsify these constraints. and then at the end, check the $clue constraint. I think this in effect does the same process as above, except it doesn't store all the combs in memory. 

// The constraints are thus next digit equals current or next digit does not equal current. And for first digit, digit not equal 0. So this is fairly straight-forward to implement using back tracking

// e.g. the constraint will look like this: !0, =, =, !=, !0, !=, =, !=, =

// we need a lexer to eat the two lines and spit out the constraints

// it'd be easier if we represent the first constraint as s s s d s s d s d s s. Same, Different
// !0 occurs either initially or after a *
//
