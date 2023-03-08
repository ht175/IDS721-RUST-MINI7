use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("data.csv")?;
    let reader = BufReader::new(file);

    // Read the CSV data into a Vec<Vec<f64>>
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<f64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        data.push(row);
    }

    // Calculate the mean of each column
    let num_rows = data.len();
    let num_cols = data[0].len();
    let mut column_sums = vec![0.0; num_cols];
    for row in &data {
        for (i, &value) in row.iter().enumerate() {
            column_sums[i] += value;
        }
    }
    let column_means: Vec<f64> = column_sums
        .iter()
        .map(|&sum| sum / num_rows as f64)
        .collect();

    // Print the column means
    for (i, &mean) in column_means.iter().enumerate() {
        println!("Column {}: mean = {}", i, mean);
    }

    Ok(())
}
