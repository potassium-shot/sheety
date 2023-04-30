extern crate sheety;

use sheety::*;

fn main() {
    let sussypiss_prime = Sprite::load("sheety/tests/sussyphus-prime.jpeg").unwrap();
    let three_sussyphuses = UnorderedSpriteSheet::new(vec![
        sussypiss_prime.clone(),
        sussypiss_prime.clone(),
        sussypiss_prime,
    ])
    .unwrap();

    let mario = Sprite::load("sheety/tests/mario-statue.png").unwrap();
    let four_marios =
        UnorderedSpriteSheet::new(vec![mario.clone(), mario.clone(), mario.clone(), mario])
            .unwrap();

    // packed with prio on columns
    SpriteSheet::concat(
        vec![three_sussyphuses, four_marios].into_iter(),
        Distribution::Packed(false),
    )
    .unwrap()
    .into_image()
    .save("sheety/tests/result.png")
    .unwrap();
}
