mod bitfield;

trait TemplateFormat {
    fn parse_input_patterns() -> PatternData;
}

trait OutputFormat {
    type Output;
    fn parse_output_shape(wave: Wave) -> Self::Output;
}

struct Pattern {
    //pattern_data: T
    occurences: usize,
}

struct PatternData {
    order: usize,
    patterns: Vec<Pattern>,
    // [edge][pattern][mask]
    adjacencies: Vec<Vec<bitfield::BitField>>,
}



struct WaveCell {
    state: bitfield::BitField,
    adjacencies: Vec<Option<usize>>,
}

struct Wave {
    order: usize,
    cells: Vec<WaveCell>,
}

impl Wave {
    fn collapse_wave(template: PatternData, wave: &mut Wave){
        if(wave.order >= template.order){
            panic!("Wave order should never be greater than template order.")
        }
    }
}