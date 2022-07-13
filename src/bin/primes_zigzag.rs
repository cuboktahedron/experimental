use graphics::ulam::tile::square_zigzag::SquareZigzag;
use graphics::ulam::generator::primes::PrimesGenerator;
use plotters::prelude::*;

// const OUT_FILE_NAME: &'static str = "output/ulam_spirals.gif";
const OUT_FILE_NAME: &'static str = "output/primes_zigzag.gif";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_size = 1200;
    let root =
        BitMapBackend::gif(OUT_FILE_NAME, (image_size, image_size), 500)?.into_drawing_area();

    let (upper, lower) = root.split_vertically(100);

    let size = ((image_size - 100) / 2) as f64;
    let chart = ChartBuilder::on(&lower)
        .margin(20)
        .build_cartesian_2d(-size..size, -size..size)?;
    let plotting_area = chart.plotting_area();

    let gen = PrimesGenerator::new(100000, 0);
    let mut ss = SquareZigzag::new(gen, &plotting_area);
    let mut i = 1;

    root.fill(&WHITE)?;

    let style = TextStyle::from(("sans-serif", 40).into_font()).color(&BLACK);
    while let Some(result) = ss.draw_next() {
        result?;

        if i < 10 {
            root.present()?;
            println!("{}", i);
            upper.fill(&WHITE)?;
            upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        } else if i <= 100 && i % 10 == 0 {
            root.present()?;
            println!("{}", i);
            upper.fill(&WHITE)?;
            upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        } else if i <= 1000 && i % 100 == 0 {
            root.present()?;
            println!("{}", i);
            upper.fill(&WHITE)?;
            upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        } else if i <= 10000 && i % 1000 == 0 {
            root.present()?;
            println!("{}", i);
            upper.fill(&WHITE)?;
            upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        } else if i <= 100000 && i % 10000 == 0 {
            root.present()?;
            println!("{}", i);
            upper.fill(&WHITE)?;
            upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        } else if i <= 1000000 && i % 100000 == 0 {
            root.present()?;
            println!("{}", i);
            upper.fill(&WHITE)?;
            upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        } else if i <= 10000000 && i % 1000000 == 0 {
            root.present()?;
            println!("{}", i);
            upper.fill(&WHITE)?;
            upper.draw_text(&format!("i = {}", i.to_string()), &style, (20, 20))?;
        }

        i += 1;
    }

    // for _ in 0..100 {
    //     root.present()?;
    // }

    Ok(())
}
