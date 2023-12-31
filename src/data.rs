use rand::Rng;

pub fn prepared_data() -> Vec<[i32; 5]> {
    return vec![
        [370702604, 532297265, 432940147, 416460079, 326591163],
        [171404959, 430210773, 675800973, 293401214, 114638532],
        [155664604, 774336904, 892781346, 234654845, 753540046],
        [258587638, 338697007, 796055231, 611209061, 259361578],
    ];
}

pub fn random_data() -> Vec<[i32; 5]> {
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
