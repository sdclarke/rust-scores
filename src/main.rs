use std::collections::HashMap;
use std::convert::TryFrom;
use std::env::args;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
/// This enum represents a score read in from a file
enum Score {
    /// The Name variant is for when a test has been missed so only the name of the person taking
    /// it is present
    Name(String),
    /// The WithScore variant is for when the test has not been missed, so there is a name and a
    /// score present
    WithScore(String, i64),
}

impl TryFrom<&str> for Score {
    type Error = Box<dyn Error>;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // If it has a colon it is a name with a score
        if s.contains(':') {
            // Split into name and score
            let s_vec: Vec<&str> = s.split(':').collect();
            // Parse the score
            let score = s_vec[1]
                .parse::<i64>()
                .map_err(|e| format!("Error parsing number: {}", e))?;
            // Return the enum
            Ok(Self::WithScore(s_vec[0].into(), score))
        } else {
            // No score so just name
            Ok(Self::Name(s.into()))
        }
    }
}

#[derive(Debug, Default)]
struct ScoreStruct {
    total: i64,
    tests: i64,
    missed: i64,
}

impl ScoreStruct {
    fn add_score(&mut self, score: i64) {
        self.total += score;
        self.tests += 1;
    }

    fn missed_test(&mut self) {
        self.missed += 1;
    }
}

impl fmt::Display for ScoreStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tests == 1 {
            write!(f, "1 test")?;
        } else {
            write!(f, "{} tests", self.tests)?;
        }
        write!(f, " with a total score of {}.", self.total)?;
        if self.missed == 1 {
            write!(f, " They missed 1 test")
        } else {
            write!(f, " They missed {} tests", self.missed)
        }
    }
}

fn parse_file(filename: &str) -> Result<Vec<Score>, Box<dyn Error>> {
    // Read the contents of the file
    let contents = fs::read_to_string(filename)?;
    contents
        .trim_end() // Remove trailing whitespace
        .split('\n') // Split into lines
        .map(Score::try_from) // Map each one to Score
        .collect() // Collect as Result<Vec<Score>, Error>>
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename which will be the second element of the args
    let filename = args().nth(1).ok_or("Expected filename")?;
    let scores = parse_file(&filename)?;
    println!("Vector: {:?}", scores);

    let mut map: HashMap<String, ScoreStruct> = HashMap::new();

    for score in scores {
        match score {
            Score::Name(name) => {
                // Get the entry or create a default entry if it isn't present, then add the missed
                // test
                map.entry(name).or_default().missed_test();
            }
            Score::WithScore(name, test_score) => {
                // Get the entry or create a default entry if it isn't present, then add the new
                // score
                map.entry(name).or_default().add_score(test_score);
            }
        }
    }
    // Commenting these out just to make stdout a bit cleaner, but I did check I could print the
    // Map
    //println!();
    //println!("Map: {:?}", map);
    println!();

    for (k, v) in map {
        println!("{} took {}", k, v);
    }

    Ok(())
}
