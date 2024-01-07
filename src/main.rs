use clap::Parser;

#[derive(Parser)]
struct Cli {
    accel_factor: f64,

    /// Max gain
    cap: f64,

    #[clap(default_value = "0")]
    input_offset: u64,

    #[clap(short, default_value = "1")]
    step: f64,

    /// Print an xorg .conf configuring libinput accel settings.
    #[clap(short = 'x', long)]
    print_xorg_conf: bool,

    #[clap(short)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();
    let accel_factor = cli.accel_factor;
    let max_gain = cli.cap;
    let step = cli.step;

    let sample_sequence = 0..64u64;

    let coordinates = sample_sequence.map(|x| x as f64 * step).map(move |x| {
        let gain = x * accel_factor + 1f64;
        let mut y = x * gain;
        if gain >= max_gain {
            y = max_gain * x
        };
        return (x, y);
    });

    let coordinates = (0..cli.input_offset)
        .map(|x| (x as f64, 0f64))
        .chain(coordinates)
        .take(64);

    if !cli.quiet {
        eprintln!("coordinates: ");
        coordinates
            .clone()
            .for_each(|(x, y)| eprint!("({x}, {y}), "));

        eprintln!();
        eprintln!();
    }

    let motion_points = coordinates
        .map(|(_, y)| format!("{}", y))
        .collect::<Vec<_>>()
        .join(" ");

    if cli.print_xorg_conf {
        println!(
            r#"Section "InputClass"
    Identifier "My Mouse"
    Driver "libinput"
    MatchIsPointer "yes"

    Option "AccelProfile" "custom"
    Option "AccelStepMotion" "{}"
    Option "AccelPointsMotion" "{}"
EndSection
"#,
            cli.step, motion_points
        )
    } else {
        println!("{motion_points}");
    }
}
