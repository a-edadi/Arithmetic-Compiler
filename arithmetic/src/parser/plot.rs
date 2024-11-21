use super::{
    ast::ASTNode, generate_random_4_digits, get_and_parse_user_input, CompilerError, Evaluator,
    Num, PlottingError, RootFinder, VariableManager,
};
use plotters::prelude::*;
use std::fs;

pub struct FunctionPlotter<'a> {
    ast: &'a ASTNode,
    vars: &'a mut VariableManager,
}

impl<'a> FunctionPlotter<'a> {
    pub fn new(ast: &'a ASTNode, vars: &'a mut VariableManager) -> Self {
        Self { ast, vars }
    }

    fn evaluate_with_x(&mut self, x: f64) -> Result<f64, CompilerError> {
        let mut evaluator = Evaluator::new(self.vars);
        evaluator.vars.set("x".to_string(), Num::Float(x));
        evaluator.evaluate(self.ast)
    }

    pub fn plot_function(&mut self) -> Result<(), CompilerError> {
        // Get user input
        let a = get_and_parse_user_input("a (start of interval)");
        let b = get_and_parse_user_input("b (end of interval)");

        // Generate x values
        let sample_points = 1000;
        let x_values: Vec<f64> = (0..=sample_points)
            .map(|i| a + (b - a) * (i as f64 / sample_points as f64))
            .collect();

        // Pre-calculate all y values
        let mut y_values = Vec::with_capacity(x_values.len());
        for &x in x_values.iter() {
            y_values.push(self.evaluate_with_x(x).unwrap_or(0.0));
        }

        // Create a new evaluator for root finding
        let mut evaluator = Evaluator::new(self.vars);
        let mut root_finder = RootFinder::new(self.ast, &mut evaluator);
        let roots = root_finder
            .find_roots(Some(a), Some(b))
            .map_err(|_| CompilerError::Plot(PlottingError::GenericPlottingError))?;

        // Pre-calculate root y-values
        let mut root_points = Vec::new();
        for &root in roots.iter() {
            let y = self.evaluate_with_x(root).unwrap_or(0.0);
            root_points.push((root, y));
        }

        // Calculate y range
        let y_min = y_values.iter().copied().reduce(f64::min).unwrap_or(0.0);
        let y_max = y_values.iter().copied().reduce(f64::max).unwrap_or(0.0);
        let y_padding = (y_max - y_min) * 0.1;

        // Create plots directory
        fs::create_dir_all("plots")
            .map_err(|_| CompilerError::Plot(PlottingError::FileCreationError))?;

        // Generate random filename
        let random_name = format!("plot_{}.png", generate_random_4_digits());
        let file_path = format!("plots/{}", random_name);

        // Create the plot
        let root = BitMapBackend::new(&file_path, (1920, 1080)).into_drawing_area();
        root.fill(&WHITE)
            .map_err(|_| CompilerError::Plot(PlottingError::GenericPlottingError))?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Function Plot", ("sans-serif", 40).into_font())
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(a..b, (y_min - y_padding)..(y_max + y_padding))
            .map_err(|_| CompilerError::Plot(PlottingError::GenericPlottingError))?;

        // Draw mesh
        chart
            .configure_mesh()
            .x_labels(10)
            .y_labels(10)
            .draw()
            .map_err(|_| CompilerError::Plot(PlottingError::GenericPlottingError))?;

        // Draw function
        chart
            .draw_series(LineSeries::new(
                x_values.into_iter().zip(y_values.into_iter()),
                &RED,
            ))
            .map_err(|_| CompilerError::Plot(PlottingError::GenericPlottingError))?;

        // Draw roots
        if !root_points.is_empty() {
            chart
                .draw_series(
                    root_points
                        .iter()
                        .map(|&(x, y)| Circle::new((x, y), 5, RED.filled())),
                )
                .map_err(|_| CompilerError::Plot(PlottingError::GenericPlottingError))?;
        }

        root.present()
            .map_err(|_| CompilerError::Plot(PlottingError::GenericPlottingError))?;

        println!("Plot saved as {}", file_path);

        Ok(())
    }
}
