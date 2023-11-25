use clap::Parser;

#[derive(Parser)]
struct Cli {
    accel_factor: f64,

    /// Max gain
    cap: f64,

    #[clap(default_value = "1")]
    step: f64,

    #[clap(short)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();
    let accel_factor = cli.accel_factor;
    let max_gain = cli.cap;
    let step = cli.step;

    let sample_sequence = 0..64u64;

    let mut hit_max_gain = false;

    let coordinates = sample_sequence.map(|x| x as f64 * step).map(move |x| {
        let gain = x * accel_factor + 1f64;
        let mut y = x * gain;
        if gain >= max_gain {
            if !hit_max_gain && !cli.quiet {
                eprintln!("gain reached {} at ({x}, {y})", gain);
            };
            hit_max_gain = true;
            y = max_gain * x
        };
        return (x, y);
    });

    if !cli.quiet {
        eprintln!("coordinates: ");
        coordinates
            .clone()
            .for_each(|(x, y)| eprint!("({x}, {y}), "));

        eprintln!();
        eprintln!();
    }

    coordinates.for_each(|(_, y)| print!("{y} "));
}
