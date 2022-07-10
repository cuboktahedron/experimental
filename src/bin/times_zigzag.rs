use graphics::ulam::cover::square_zigzag::SquareZigzag;
use graphics::ulam::generator::times::TimesGenerator;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_size = 1600;
    let style = TextStyle::from(("sans-serif", 40).into_font()).color(&BLACK);

    for i in 1..=256 {
        let file_name = format!("output/times_zigzag/{}.png", i);
        let root = BitMapBackend::new(&file_name, (image_size, image_size)).into_drawing_area();
        root.fill(&WHITE)?;

        let (upper, lower) = root.split_vertically(100);

        let size = ((image_size - 100) / 2) as f64;
        let chart = ChartBuilder::on(&lower)
            .margin(20)
            .build_cartesian_2d(-size..size, -size..size)?;
        let plotting_area = chart.plotting_area();

        let gen = TimesGenerator::new(1000000, i, 0);
        let mut ss = SquareZigzag::new(gen, &plotting_area);

        while let Some(result) = ss.draw_next() {
            result?;
        }

        upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        root.present()?;

        println!("times {} done", i);
    }

    Ok(())
}
