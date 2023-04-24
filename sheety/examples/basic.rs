extern crate sheety;

use sheety::{error::*, *};

fn main() {
    let mut sheet = SpriteSheet::new(IVec2::new(2, 1), IVec2::new(256, 256));
    let sussypiss_prime = Sprite::load("sheety/tests/sussyphus-prime.jpeg").unwrap();
    let three_sussyphuses = UnorderedSpriteSheet::new(vec![
        sussypiss_prime.clone(),
        sussypiss_prime.clone(),
        sussypiss_prime,
    ]);

    match sheet.push_sprites(three_sussyphuses).unwrap_err() {
        Error::SheetFull { amount_fitted } => assert_eq!(amount_fitted, 2),
        other => panic!("expected Error::SheetFull, got {:?}", other),
    }
}
