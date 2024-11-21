use super::{generate_random_4_digits, get_and_parse_user_input, ASTWrapper};
use plotters::prelude::*;
use std::fs;

impl ASTWrapper {
    pub fn plot_function(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the user-provided interval values
        let a = get_and_parse_user_input("a (start of interval)");
        let b = get_and_parse_user_input("b (end of interval)");

        // Set sample points
        let sample_points = 1000;

        let x_values: Vec<f64> = (0..=sample_points)
            .map(|i| a + (b - a) * (i as f64 / sample_points as f64))
            .collect();

        // Evaluate function values with more points
        let y_values: Vec<f64> = x_values
            .iter()
            .map(|&x| self.evaluate_with_x(x).unwrap_or(0.0))
            .collect();

        // Determine y-range
        let y_min = y_values.iter().copied().reduce(f64::min).unwrap_or(0.0);
        let y_max = y_values.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let y_padding = (y_max - y_min) * 0.1;

        // Ensure the "plots" directory exists
        fs::create_dir_all("plots")?;

        // Generate a random file name for the plot
        let random_name: String = {
            let random_number = generate_random_4_digits();
            format!("plot_{}.png", random_number as u64)
        };
        let file_path = format!("plots/{}", random_name);

        // Create plot
        let roots = self.find_roots(Some(a), Some(b))?;

        let root = BitMapBackend::new(&file_path, (1920, 1080)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Function Plot", ("sans-serif", 40).into_font())
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(a..b, (y_min - y_padding)..(y_max + y_padding))?;

        // Configure mesh with more labels
        chart.configure_mesh().x_labels(20).y_labels(20).draw()?;

        // Draw the function curve with higher resolution
        chart.draw_series(LineSeries::new(
            x_values.into_iter().zip(y_values.into_iter()),
            &RED,
        ))?;

        // Draw roots if they exist (no label)
        if !roots.is_empty() {
            chart.draw_series(roots.iter().map(|&root| {
                Circle::new(
                    (root, self.evaluate_with_x(root).unwrap_or(0.0)),
                    5,
                    RED.filled(),
                )
            }))?;
        }

        root.present()?;
        println!("Plot saved as {}", file_path);

        Ok(())
    }
}
