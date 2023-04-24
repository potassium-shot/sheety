use sheety::{error::Error, *};

#[test]
fn push_one() {
    let mut sheet = SpriteSheet::new(IVec2::new(5, 5), IVec2::new(256, 256));
    let sussypiss_prime = Sprite::load("tests/sussyphus-prime.jpeg").unwrap();
    sheet.push_sprite(sussypiss_prime).unwrap();

    assert!(sheet.get_cell(IVec2::new(0, 0)).unwrap().is_sprite());
    assert!(sheet.get_cell(IVec2::new(1, 0)).unwrap().is_empty());
    sheet.get_cell(IVec2::new(5, 0)).unwrap_err();
}

#[test]
fn push_several() {
    let mut sheet = SpriteSheet::new(IVec2::new(2, 1), IVec2::new(256, 256));
    let sussypiss_prime = Sprite::load("tests/sussyphus-prime.jpeg").unwrap();
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
