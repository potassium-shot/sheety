extern crate sheety;

use sheety::*;

fn main() {
    let sussypiss_prime = Sprite::load("sheety/examples/assets/sussyphus-prime.jpeg").unwrap();
    let three_sussyphuses = UnorderedSpriteSheet::new(vec![
        sussypiss_prime.clone(),
        sussypiss_prime.clone(),
        sussypiss_prime,
    ])
    .unwrap();

    let mario = Sprite::load("sheety/examples/assets/mario-statue.png").unwrap();
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
    .save("sheety/examples/assets/result.png")
    .unwrap();
}
