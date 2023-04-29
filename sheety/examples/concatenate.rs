use sheety::*;

fn main() {
    SpriteSheet::concat(
        vec![
            SpriteSheet::load_cell_size("sheety/tests/machete_idle.png", (256, 256))
                .unwrap()
                .into_unordered()
                .unwrap(),
            SpriteSheet::load_cell_size("sheety/tests/machete_swing01.png", (256, 256))
                .unwrap()
                .into_unordered()
                .unwrap(),
        ]
        .into_iter(),
        Distribution::Packed(false),
    )
    .unwrap()
    .save("sheety/tests/result_machete.png")
    .unwrap();
}
