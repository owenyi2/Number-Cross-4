use std::fs::File;
use std::io::Write;

struct Rec {
    strings: Vec<String>,
    templates: [&'static str; 9],
    delimiter: &'static str,
    string_length: i32,
}

impl Rec {
    fn new() -> Rec {
        Rec {
            strings: Vec::new(),
            templates: [
                "__",
                "___",
                "____",
                "_____",
                "______",
                "_______",
                "________",
                "__________",
                "___________",
            ],
            delimiter: "*",
            string_length: 11,
        }
    }
    fn rec(&mut self, string: String) {
        let candidate_lengths = 2..(self.string_length - string.len() as i32 + 1);
        
        if candidate_lengths.is_empty() {
            if string.len() as i32 == self.string_length {
                self.strings.push(string)
            } 
        } else {
            for length in candidate_lengths {
                for template in self.templates {
                    let next_string = string.clone() + template;
                    if next_string.len() as i32 == self.string_length {
                        self.rec(next_string);
                    } else {
                        self.rec(next_string + self.delimiter);
                    }
                }
            } 
        }
    }
    fn run(&mut self) {
        self.rec(String::from(""));
        self.rec(String::from(self.delimiter));
    }
}

fn main() -> std::io::Result<()> {
    let mut r = Rec::new();
    r.run();

    let mut file = File::create("templates.txt")?;
    for s in r.strings {file.write(s.as_bytes()); file.write(b"\n"); }
    Ok(())
}

// our code here sucks. We repeated sequences a bunch of times for no reason.
// wc -l templates.txt gives 4363 templates.txt
// sort templates.txt | uniq | wc -l gives 53


// ==== //

// For anything less than 9, it is feasible to pregenerate and filter as required
// For anything longer, it is far better to generate within constraints


