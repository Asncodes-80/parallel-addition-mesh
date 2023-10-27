use rand::Rng;

pub fn prepared_data() -> Vec<[i32; 5]> {
    let mut rng = rand::thread_rng();

    let data = vec![
        [
            rng.gen_range(100_000_000..=500_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
        ],
        [
            rng.gen_range(100_000_000..=500_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
        ],
        [
            rng.gen_range(100_000_000..=500_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
        ],
        [
            rng.gen_range(100_000_000..=500_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
            rng.gen_range(100_000_000..=900_000_000),
        ],
    ];

    return data;
}
