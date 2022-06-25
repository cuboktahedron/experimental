use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "output/ulam_spiral.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (800, 800)).into_drawing_area();

    root.fill(&WHITE)?;

    let size = 800f64;
    let chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_cartesian_2d(-size..size, -size..size)?;

    let plotting_area = chart.plotting_area();

    // let range = plotting_area.get_pixel_range();
    //    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);

    let block = 16f64;

    for (_no, x, y, is_prime) in spiral(primes(0, 10000)) {
        let coord1 = ((x as f64 * block), (y as f64 * block));
        let coord2 = ((x + 1) as f64 * block, (y + 1) as f64 * block);

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

        plotting_area.draw(&Rectangle::new(
            [coord1, coord2],
            Into::<ShapeStyle>::into(&BLACK),
        ))?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect(
        "Unable to write result to file, please make sure 'output' dir exists under current dir",
    );
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn primes(from: usize, to: usize) -> impl Iterator<Item = (usize, bool)> {
    primes_or_not(to).into_iter().enumerate().skip(from)
}

fn spiral(
    data: impl Iterator<Item = (usize, bool)>,
) -> impl Iterator<Item = (usize, isize, isize, bool)> {
    let mut v = vec![];
    // (dir, rest, step)
    // 0: up, 1: left, 2: down, 3: right
    let mut ctx = (3usize, 1isize, 1isize);
    let mut prev = (0, -1, 0, false);

    for (n, b) in data {
        let (dir, mut rest, step) = ctx;
        let (_, x, y, _) = prev;

        if dir == 0 {
            v.push((n, x, y + 1, b));
        } else if dir == 1 {
            v.push((n, x - 1, y, b));
        } else if dir == 2 {
            v.push((n, x, y - 1, b));
        } else {
            v.push((n, x + 1, y, b));
        }

        prev = v[v.len() - 1];
        rest -= 1;

        if rest == 0 {
            if dir == 0 {
                ctx = (1, step + 1, step + 1);
            } else if dir == 1 {
                ctx = (2, step, step);
            } else if dir == 2 {
                ctx = (3, step + 1, step + 1);
            } else {
                ctx = (0, step, step);
            }
        } else {
            ctx = (dir, rest, step);
        }
    }

    v.into_iter()
}

// mod ulam {
//     struct PrimesOrNotIterator<T: Ord + Clone> {
//         i: usize,
//         is_finished: bool,
//     }
//     impl <T: Ord + Clone> PrimesOrNotIterator<T> {
//         pub fn new(n: usize) -> PrimesOrNotIterator<T> {
//             PrimesOrNotIterator {
//                 i: n,
//                 is_finished: false,
//             }
//         }
//         impl <T: Ord + Clone> Iterator for PrimesOrNotIterator {
//         }
//     }
// }

mod perm {
    pub struct PrimesOrNotIterator {
        i: usize,
        max: usize,
        primes: Vec<usize>,
    }

    impl PrimesOrNotIterator {
        pub fn new(n: usize) -> PrimesOrNotIterator {
            PrimesOrNotIterator {
                i: 0,
                max: n,
                primes: vec![],
            }
        }
    }

    impl Iterator for PrimesOrNotIterator {
        type Item = (usize, bool);

        fn next(&mut self) -> Option<Self::Item> {
            let i = self.i;
            if self.i > self.max {
                return None;
            }

            if self.i < 2 {
                self.i += 1;
                return Some((i, false));
            }

            let mut n = i;
            for &p in &self.primes {
                while n % p == 0 {
                    n /= p;
                }
            }

            self.i += 1;

            if n != 1 {
                self.primes.push(i);
                Some((i, true))
            } else {
                Some((i, false))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_enumerate_bits() {
            let mut ite = PrimesOrNotIterator::new(5);
            assert_eq!(ite.next(), Some((0, false)));
            assert_eq!(ite.next(), Some((1, false)));
            assert_eq!(ite.next(), Some((2, true)));
            assert_eq!(ite.next(), Some((3, true)));
            assert_eq!(ite.next(), Some((4, false)));
            assert_eq!(ite.next(), Some((5, true)));
            assert_eq!(ite.next(), None);
        }
    }
}

fn primes_or_not(n: usize) -> Vec<bool> {
    let mut pn = vec![true; n + 1];
    pn[0] = false;
    pn[1] = false;
    let mut i = 2;
    while i * i <= n {
        if pn[i] {
            for j in (i + i..=n).step_by(i) {
                pn[j] = false;
            }
        }
        i += 1;
    }
    pn
}
