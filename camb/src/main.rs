use rand::prelude::*;
use std::time::{Duration, Instant};

#[allow(non_upper_case_globals)]
const Ki: usize = 1024;

#[allow(non_upper_case_globals)]
const Mi: usize = 1024 * Ki;

#[allow(non_upper_case_globals)]
const Gi: usize = 1024 * Mi;

fn read(buffer: &[i64]) -> i64 {
    buffer.iter().step_by(64 / std::mem::size_of::<i64>()).sum()
}

fn write(buffer: &mut [i64]) {
    buffer
        .iter_mut()
        .step_by(64 / std::mem::size_of::<i64>())
        .for_each(|x| *x = 0);
}

fn make_buffer(size: usize) -> Vec<i64> {
    let mut buffer = vec![0; size / std::mem::size_of::<i64>()];
    let mut rng = thread_rng();
    for x in buffer.iter_mut() {
        *x = rng.gen();
    }
    buffer
}

fn time(mut f: impl FnMut() -> ()) -> Duration {
    let then = Instant::now();

    f();

    Instant::now().duration_since(then)
}

fn main() {
    let mut buffer = make_buffer(512 * Mi);

    #[rustfmt::skip]
    let sizes = [
        512 * Mi, 256 * Mi, 128 * Mi, 64 * Mi,
        32 * Mi, 16 * Mi, 24 * Mi, 12 * Mi,
        8 * Mi, 6 * Mi, 4 * Mi, 2 * Mi,
        1 * Mi, 512 * Ki, 256 * Ki, 192 * Ki,
        128 * Ki, 64 * Ki, 48 * Ki, 32 * Ki,
        24 * Ki, 16 * Ki, 12 * Ki, 8 * Ki,
    ];

    let total_size = 16 * Gi;

    for size in sizes.iter() {
        let slice = size / std::mem::size_of::<i64>();
        let iterations = total_size / size;

        let mut side_effect = 0;

        let reads = time(|| {
            for _ in 0..iterations {
                side_effect = read(&buffer[0..slice]);
            }
        });
        let reads = total_size as f64 / reads.as_nanos() as f64;

        let writes = time(|| {
            for _ in 0..iterations {
                write(&mut buffer[0..slice]);
            }
        });
        let writes = total_size as f64 / writes.as_nanos() as f64;

        let hsize = if *size < 1 * Mi {
            format!("{} KiB", size / Ki)
        } else if *size < 1 * Gi {
            format!("{} MiB", size / Mi)
        } else {
            format!("{} GiB", size / Gi)
        };

        println!(
            "{} => {:.1} GiB/s reads, {:.1} GiB/s writes [{}]",
            hsize, reads, writes, side_effect
        );
    }
}
