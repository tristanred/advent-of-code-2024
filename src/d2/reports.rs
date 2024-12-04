use std::{io::Read, os::unix::fs::DirEntryExt};

struct Report {
    pub readings: Vec<u64>,
}

impl Report {
    pub fn from_string_list(line: &str) -> Self {
        let values = line
            .split_whitespace()
            .collect::<Vec<&str>>();

        let cast = values
            .iter()
            .map(|f| {
                f.parse()
                    .expect("Unable to cast reading into u64.")
            })
            .collect();

        Report { readings: cast }
    }

    pub fn is_safe(&self) -> bool {
        let windows = self.readings.windows(2);

        // Current direction for the whole report. Starts unitialized and
        // gets set up on the first pair. -1 for ascending, 1 for descending
        let mut direction: Option<i64> = None;

        for pair in windows {
            let a = pair[0] as i64;
            let b = pair[1] as i64;

            // Check threshold
            let diff = a.abs_diff(b);
            if diff > 3 || diff == 0 {
                return false;
            }

            // Check if direction is maintained
            match direction {
                Some(d) => {
                    let window_dir = (a - b).signum();

                    if window_dir != d {
                        return false;
                    }
                }
                None => {
                    // n^0 will either give 1 or -1=
                    direction = Some((a - b).signum());
                }
            }
        }

        return true;
    }
}

pub fn execute_p1() {
    // Idea 1
    // Read input
    let contents = read_input("src/d2/input.txt").expect("Unable to read input file.");

    let reports = contents
        .lines()
        .map(|f| Report::from_string_list(f))
        .collect::<Vec<Report>>();

    let safe_reports = reports
        .iter()
        .filter(|f| f.is_safe())
        .collect::<Vec<&Report>>();

    let count = safe_reports.len();

    println!("Day 2, Part 1. Result is {}", count)
}

fn read_input(path: &str) -> Result<String, std::io::Error> {
    let mut contents = String::new();

    let _f = std::fs::File::open(path)?.read_to_string(&mut contents)?;

    return Ok(contents);
}
