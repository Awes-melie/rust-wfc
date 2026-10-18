mod bitfield;


// input
// input parser => index + adjacencies + og value + occurrences
trait ToRepresentation {}

struct Adjacency {
    index: usize,
    side: u32,
    occurrences: u32,
}

struct Adjacencies(Vec<Adjacency>);

struct SubPattern {
    //pattern_data: ndarray,
    side_adjacencies: Vec<Adjacencies>,
}

impl SubPattern {
    fn occurrences(&self) -> u32 {
        self.side_adjacencies.iter().fold(0, |acc, side| {
            side.0
                .iter()
                .fold(0, |acc2, adjacency| adjacency.occurrences + acc2)
                + acc
        })
    } // need to git add lib rs? also maybe need to add to pckg?
}

struct InputPatterns {
    sub_patterns: Vec<SubPattern>,
}

struct InputData {
    //ndarray
    // ...
}

fn parse (InputData) -> InputPatterns {

}