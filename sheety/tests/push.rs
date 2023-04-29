use sheety::{error::Error, *};

#[test]
fn push_one() {
    let mut sheet = SpriteSheet::new((5, 5), (256, 256));
    let sussypiss_prime = Sprite::load("tests/sussyphus-prime.jpeg").unwrap();
    sheet.push_sprite(sussypiss_prime).unwrap();

    assert!(sheet.get_cell((0, 0)).unwrap().is_sprite());
    assert!(sheet.get_cell((1, 0)).unwrap().is_empty());
    sheet.get_cell((5, 0)).unwrap_err();
}

#[test]
fn push_several() {
    let mut sheet = SpriteSheet::new((2, 1), (256, 256));
    let sussypiss_prime = Sprite::load("tests/sussyphus-prime.jpeg").unwrap();
    let three_sussyphuses = UnorderedSpriteSheet::new(vec![
        sussypiss_prime.clone(),
        sussypiss_prime.clone(),
        sussypiss_prime,
    ])
    .unwrap();

    match sheet.push_sprites(three_sussyphuses).unwrap_err() {
        Error::SheetFull { amount_fitted } => assert_eq!(amount_fitted, 2),
        other => panic!("expected Error::SheetFull, got {:?}", other),
    }
}
