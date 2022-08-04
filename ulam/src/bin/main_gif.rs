use clap::Parser;
use plotters::coord::types::RangedCoordf64;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::BitMapBackend;
use plotters::prelude::Cartesian2d;
use plotters::prelude::ChartBuilder;
use plotters::prelude::DrawingArea;
use plotters::prelude::TextStyle;
use plotters::prelude::BLACK;
use plotters::prelude::WHITE;
use plotters::style::IntoFont;
use std::fs::create_dir;
use std::fs::create_dir_all;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use ulam::ulam::generator::generator::Generator;
use ulam::ulam::generator::prime1s::Prime1sGenerator;
use ulam::ulam::generator::prime3s::Prime3sGenerator;
use ulam::ulam::generator::prime7s::Prime7sGenerator;
use ulam::ulam::generator::primes::PrimesGenerator;
use ulam::ulam::generator::squares::SquareGenerator;
use ulam::ulam::generator::times::TimesGenerator;
use ulam::ulam::tile::hexagon_spiral::HexagonSpiral;
use ulam::ulam::tile::square_spiral::SquareSpiral;
use ulam::ulam::tile::square_zigzag::SquareZigzag;
use ulam::ulam::tile::tile::Tile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg: AppArg = AppArg::parse();

    if !Path::new("output").is_dir() {
        create_dir("output")?;
    }
    let file_path = if let Some(ref file) = arg.output {
        format!("output/{}", file)
    } else {
        format!(
            "output/{}-{}-{}-{}.gif",
            arg.generator, arg.tile, arg.image_size, arg.gp
        )
    };

    let path = Path::new(&file_path);
    if let Some(ref parent) = path.parent() {
        if !parent.is_dir() {
            println!("{:?}", parent);
            create_dir_all(parent).expect(&format!("Can't create dir({:?})", parent));
        }
    }

    let root = BitMapBackend::gif(&file_path, (arg.image_size, arg.image_size), arg.interval)?
        .into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(100);

    let size = ((arg.image_size - 100) / 2) as f64;
    let chart = ChartBuilder::on(&lower)
        .margin(20)
        .build_cartesian_2d(-size..size, -size..size)?;
    let plotting_area = chart.plotting_area();

    let style = TextStyle::from(("sans-serif", 30).into_font()).color(&BLACK);

    let gen = create_generator(&arg)?;
    let generator_info = gen.generator_info();

    let mut tile = create_tile(&arg, gen, &plotting_area)?;
    let tile_info = tile.tile_info();

    let mut animation = Animation::new(&arg.animation)?;
    let mut n = 0;
    while let Some(result) = tile.draw_next() {
        n = result?;

        if animation.next() {
            upper.fill(&WHITE)?;
            upper.draw_text(&generator_info, &style, (20, 10))?;
            upper.draw_text(&tile_info, &style, (20, 40))?;
            upper.draw_text(&format!("n = {}", n), &style, (20, 70))?;
            root.present()
                .expect(&format!("Failed to output file({})", file_path));
        }
    }

    upper.fill(&WHITE)?;
    upper.draw_text(&generator_info, &style, (20, 10))?;
    upper.draw_text(&tile_info, &style, (20, 40))?;
    upper.draw_text(&format!("n = {}", n), &style, (20, 70))?;
    root.present()
        .expect(&format!("Failed to output file({})", file_path));

    let wait_num = (arg.wait_after + arg.interval - 1) / arg.interval;

    for _ in 0..wait_num {
        root.present()
            .expect(&format!("Failed to output file({})", file_path));
    }

    Ok(())
}

fn create_generator(arg: &AppArg) -> Result<Box<dyn Generator>, Box<dyn std::error::Error>> {
    if arg.generator == "primes" {
        let gen = PrimesGenerator::from_gp(&arg.gp)?;
        return Ok(Box::new(gen));
    }

    if arg.generator == "prime1s" {
        let gen = Prime1sGenerator::from_gp(&arg.gp)?;
        return Ok(Box::new(gen));
    }

    if arg.generator == "prime3s" {
        let gen = Prime3sGenerator::from_gp(&arg.gp)?;
        return Ok(Box::new(gen));
    }

    if arg.generator == "prime7s" {
        let gen = Prime7sGenerator::from_gp(&arg.gp)?;
        return Ok(Box::new(gen));
    }

    if arg.generator == "times" {
        let gen = TimesGenerator::from_gp(&arg.gp)?;
        return Ok(Box::new(gen));
    }

    if arg.generator == "squares" {
        let gen = SquareGenerator::from_gp(&arg.gp)?;
        return Ok(Box::new(gen));
    }

    panic!()
}

fn create_tile<'a>(
    arg: &AppArg,
    gen: Box<dyn Generator>,
    plotting_area: &'a DrawingArea<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) -> Result<Box<dyn Tile + 'a>, Box<dyn std::error::Error>> {
    if arg.tile == "spiral4" {
        let gen = SquareSpiral::from_tp(&arg.tp, gen, plotting_area)?;
        return Ok(Box::new(gen));
    }

    if arg.tile == "spiral6" {
        let gen = HexagonSpiral::from_tp(&arg.tp, gen, plotting_area)?;
        return Ok(Box::new(gen));
    }

    if arg.tile == "zigzag4" {
        let gen = SquareZigzag::from_tp(&arg.tp, gen, plotting_area)?;
        return Ok(Box::new(gen));
    }

    panic!()
}

#[derive(Parser, Debug)]
#[clap(
    name = "Gen-Tile-gif",
    author = "cuboktahedron",
    version = "v0.1.0",
    about = "Draw with generator and tile."
)]

struct AppArg {
    #[clap(short, long, default_value = "primes")]
    generator: String,

    #[clap(short, long, default_value = "spiral4")]
    tile: String,

    #[clap(long, default_value = "0")]
    gp: String,

    #[clap(long, default_value = "0")]
    tp: String,

    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long, default_value = "800")]
    image_size: u32,

    #[clap(long, default_value = "100")]
    interval: u32,

    #[clap(long, default_value = "100:1")]
    animation: String,

    #[clap(long, default_value = "1000")]
    wait_after: u32,
}

struct Animation {
    animations: Vec<(usize, usize, usize)>,
}

impl Animation {
    fn new(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut animations = vec![];

        for data in data.split(",") {
            let mut a = data.split(":");

            let step1 = a.next();
            let step1: usize = if let Some(step1) = step1 {
                step1.parse()?
            } else {
                100
            };

            let step2 = a.next();
            let step2: usize = if let Some(step2) = step2 {
                step2.parse()?
            } else {
                100
            };

            animations.push((step1, step2, step1));
        }

        if animations.is_empty() {
            animations.push((100, !0, 100));
        }

        let last_step = animations.last_mut().unwrap();
        last_step.1 = !0;

        animations.reverse();
        Ok(Self { animations })
    }

    fn next(&mut self) -> bool {
        let mut step = self.animations.pop().unwrap();

        if step.0 > 1 {
            step.0 -= 1;
            self.animations.push(step);
            return false;
        } else if step.1 > 0 {
            step.1 -= 1;
            step.0 = step.2;
            self.animations.push(step);
            print!(".");
            stdout().flush().unwrap();
            return true;
        } else {
            print!(".");
            stdout().flush().unwrap();
            return true;
        }
    }
}
