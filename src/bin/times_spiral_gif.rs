use graphics::ulam::cover::square_spiral::SquareSpiral;

use graphics::ulam::generator::times::TimesGenerator;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_size = 1200;
    let interval = 500;
    let file_name = format!("output/times_spiral/all_{}ms.gif", interval);
    let root =
        BitMapBackend::gif(file_name, (image_size, image_size), interval)?.into_drawing_area();

    let (upper, lower) = root.split_vertically(100);

    let size = ((image_size - 100) / 2) as f64;
    let chart = ChartBuilder::on(&lower)
        .margin(20)
        .build_cartesian_2d(-size..size, -size..size)?;
    let plotting_area = chart.plotting_area();

    let style = TextStyle::from(("sans-serif", 40).into_font()).color(&BLACK);

    for i in 1..=256 {
        root.fill(&WHITE)?;

        let gen = TimesGenerator::new(100000, i, 0);
        let mut ss = SquareSpiral::new(gen, &plotting_area);

        while let Some(result) = ss.draw_next() {
            result?;
        }

        upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        root.present()?;

        println!("times {} done", i);
    }

    for _ in 0..3000 / interval {
        root.present()?;
    }

    Ok(())
}
