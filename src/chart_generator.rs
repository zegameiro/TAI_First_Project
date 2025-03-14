use std::collections::HashMap;
use plotters::prelude::*;

pub struct ChartGenerator {
    data: HashMap<char, HashMap<char, f32>>,
    alpha: f32,
    total_symbols: f32,
    symbols: Vec<char>,
    probabilities: Vec<f32>
}

impl ChartGenerator {
    pub fn new(alpha: f32, total_symbols: f32) -> Self {
        Self {
            data: HashMap::new(),
            alpha,
            symbols: Vec::new(),
            total_symbols,
            probabilities: Vec::new()
        }
    }

    pub fn compute_probability(&self, symbol: char, next_symbol: char) -> f32 {
        let binding: HashMap<char, f32> = HashMap::new();
        let symbol_counts: &HashMap<char, f32> = self.data.get(&symbol).unwrap_or(&binding);
        let symbol_count: f32 = *symbol_counts.get(&next_symbol).unwrap_or(&0.0);
        let total_count: f32 = symbol_counts.values().sum::<f32>();

        let res = (symbol_count + self.alpha) / (total_count + self.alpha * self.total_symbols as f32);

        -res.log2()
    }

    pub fn train_char(&mut self, symbol: char, next_symbol: char) {

        if !self.symbols.contains(&symbol) {
            self.symbols.push(symbol);
        }

        let prob = self.compute_probability(symbol, next_symbol);
        self.probabilities.push(prob);

        let entry = self.data.entry(symbol).or_insert_with(HashMap::new); 
        entry.entry(next_symbol).or_insert(0.0);

        *entry.get_mut(&next_symbol).unwrap() += 1.0;

    }

    pub fn draw_chart(&self, output_file: &str) {
        let root = BitMapBackend::new(output_file, (800,600)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let max_prob = self.probabilities.iter().cloned().fold(0.0/0.0, f32::max);

        let mut chart = ChartBuilder::on(&root)
            .caption("Probability Distribution", ("sans-serif", 30))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(1..(self.probabilities.len() + 1) as i32, 0.0..max_prob).unwrap();

        let _ = chart.configure_mesh()
            .x_desc("Symbol")
            .y_desc("Probability")
            .draw();

        let _ = chart.draw_series(
            self.probabilities
                .iter()
                .enumerate()
                .map(|(i, &prob)| Circle::new((i as i32 + 1, prob), 5, RED.filled())),
        );

        let _ = root.present();
        println!("Chart saved to {}", output_file);

    }

}