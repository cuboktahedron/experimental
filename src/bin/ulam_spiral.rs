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

    // let range = plotting_area.get_pixel_range();
    //    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let n = (100000f64 - 0f64).sqrt().ceil();
    let block = image_size as f64 / n;

    for (_no, x, y, is_prime) in cover_primes_squares(0, 10000) {
    // for (_no, x, y, is_prime) in cover_times_squares(0, 100000, 97) {
        let coord1 = graphics::ulam::cover::square_spiral::normalize(block, x, y);
        let coord2 = graphics::ulam::cover::square_spiral::normalize(block, x + 1, y + 1);

        if is_prime {
            plotting_area.draw(&Rectangle::new(
                [coord1, coord2],
                Into::<ShapeStyle>::into(&RED).filled(),
            ))?;

            // let pos = Pos::new(HPos::Center, VPos::Center);
            // let style = TextStyle::from(("sans-serif", 10).into_font()).pos(pos);
            // plotting_area.draw(&Text::new(
            //     no.to_string(),
            //     (coord1.0 + 8f64, coord1.1 + 8f64),
            //     &style,
            // ))?;
        }

        // plotting_area.draw(&Rectangle::new(
        //     [coord1, coord2],
        //     Into::<ShapeStyle>::into(&BLACK),
        // ))?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect(
        "Unable to write result to file, please make sure 'output' dir exists under current dir",
    );
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

#[allow(dead_code)]
fn cover_primes_squares(
    from: usize,
    to: usize,
) -> impl Iterator<Item = (usize, isize, isize, bool)> {
    let ite = graphics::ulam::generator::primes::generate(to)
        .into_iter()
        .skip(from);
    graphics::ulam::cover::square_spiral::cover(ite)
}

#[allow(dead_code)]
fn cover_times_squares(
    from: usize,
    to: usize,
    times_base: usize,
) -> impl Iterator<Item = (usize, isize, isize, bool)> {
    let ite = graphics::ulam::generator::times::generate(times_base, to)
        .into_iter()
        .skip(from);
    graphics::ulam::cover::square_spiral::cover(ite)
}
