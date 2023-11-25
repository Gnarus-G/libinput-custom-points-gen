fn main() {
    let accel_factor = 0.7f64;
    let max_gain = 2f64;
    let step = 0.1f64;

    let sample_sequence = 0..64u64;

    let mut hit_max_gain = false;

    let coordinates = sample_sequence.map(|x| x as f64 * step).map(move |x| {
        let gain = x * accel_factor + 1f64;
        let mut y = x * gain;
        if gain >= max_gain {
            if !hit_max_gain {
                eprintln!("gain reached {} at ({x}, {y})", gain);
            };
            hit_max_gain = true;
            y = max_gain * x
        };
        return (x, y);
    });

    eprintln!("coordinates: ");
    coordinates
        .clone()
        .for_each(|(x, y)| eprint!("({x}, {y}), "));

    eprintln!();
    eprintln!();

    coordinates.for_each(|(_, y)| print!("{y} "));
}
