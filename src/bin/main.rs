use clap::Parser;
use graphics::ulam::tile::square_spiral::SquareSpiral;
use graphics::ulam::generator::generator::Generator;
use graphics::ulam::generator::primes::PrimesGenerator;
use graphics::ulam::generator::times::TimesGenerator;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::BitMapBackend;
use plotters::prelude::ChartBuilder;
use plotters::prelude::TextStyle;
use plotters::prelude::BLACK;
use plotters::prelude::WHITE;
use plotters::style::IntoFont;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let arg: AppArg = AppArg::parse();

  let image_size = 1600;
  let style = TextStyle::from(("sans-serif", 40).into_font()).color(&BLACK);

  let file_name = format!("output/test.png");
  let root = BitMapBackend::new(&file_name, (image_size, image_size)).into_drawing_area();
  root.fill(&WHITE)?;

  let (upper, lower) = root.split_vertically(100);

  let size = ((image_size - 100) / 2) as f64;
  let chart = ChartBuilder::on(&lower)
    .margin(20)
    .build_cartesian_2d(-size..size, -size..size)?;
  let plotting_area = chart.plotting_area();

  let gen = create_generator(&arg)?;
  let mut ss = SquareSpiral::new(gen, &plotting_area);

  while let Some(result) = ss.draw_next() {
    result?;
  }

  upper.draw_text(&format!("{}", arg.gp), &style, (20, 20))?;
  root.present()?;

  Ok(())
}

fn create_generator(arg: &AppArg) -> Result<Box<dyn Generator>, Box<dyn std::error::Error>> {
  if arg.generator == "primes" {
    let gen = PrimesGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  if arg.generator == "times" {
    let gen = TimesGenerator::from_gp(&arg.gp)?;
    return Ok(Box::new(gen));
  }

  panic!()
}

#[derive(Parser, Debug)]
#[clap(
  name = "Gen-Cov",
  author = "cuboktahedron",
  version = "v0.1.0",
  about = "Draw with generator and tile."
)]

struct AppArg {
  #[clap(short, long, default_value = "primes")]
  generator: String,

  #[clap(short, long, default_value = "spiral4")]
  tile: String,

  #[clap(long)]
  gp: String,

  #[clap(short, long)]
  output: Option<String>,

  #[clap(short, long, default_value = "800")]
  image_size: u16,
}
