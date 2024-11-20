use super::{get_value, ASTWrapper};
use plotters::prelude::*;

impl ASTWrapper {
    pub fn plot_function(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Get the user-provided interval values
        let a = get_value("a (start of interval)");
        let b = get_value("b (end of interval)");

        // Increase sample points for smoother curve, especially for trigonometric functions
        let sample_points = 1000; // Increased from previous implementations
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

        // Create plot
        let root = BitMapBackend::new("function_plot.png", (1024, 768)).into_drawing_area();
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
        chart
            .draw_series(LineSeries::new(
                x_values.into_iter().zip(y_values.into_iter()),
                &RED,
            ))?
            .label("f(x)");

        // Add series labels
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        println!("Plot saved as function_plot.png");

        Ok(())
    }
}
