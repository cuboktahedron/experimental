use graphics::ulam::cover::square_spiral::SquareSpiral;
use graphics::ulam::generator::primes::PrimesGenerator;
use graphics::ulam::generator::times::TimesGenerator;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "output/ulam_spiral.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_size = 1600;
    let root = BitMapBackend::new(OUT_FILE_NAME, (image_size, image_size)).into_drawing_area();

    root.fill(&WHITE)?;

    let size = (image_size / 2) as f64;
    let chart = ChartBuilder::on(&root)
        .margin(20)
        .build_cartesian_2d(-size..size, -size..size)?;
    let plotting_area = chart.plotting_area();

    let mut gen = PrimesGenerator::new(10000, 0);
    let mut ss = SquareSpiral::new(&mut gen, &plotting_area);
    while let Some(result) = ss.draw_next() {
        result?
    }

    let mut gen = TimesGenerator::new(10000, 5, 0);
    let mut ss = SquareSpiral::new(&mut gen, &plotting_area);
    while let Some(result) = ss.draw_next() {
        result?
    }

    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
