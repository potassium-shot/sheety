use sheety::*;

#[test]
fn concat() {
    let sussypiss_prime = Sprite::load("examples/assets/sussyphus-prime.jpeg").unwrap();
    let three_sussyphuses = UnorderedSpriteSheet::new(vec![
        sussypiss_prime.clone(),
        sussypiss_prime.clone(),
        sussypiss_prime,
    ])
    .unwrap();

    let mario = Sprite::load("examples/assets/mario-statue.png").unwrap();
    let four_marios =
        UnorderedSpriteSheet::new(vec![mario.clone(), mario.clone(), mario.clone(), mario])
            .unwrap();

    // packed with prio on columns
    SpriteSheet::concat(
        vec![three_sussyphuses.clone(), four_marios.clone()].into_iter(),
        Distribution::Packed(false),
    )
    .unwrap();

    // packed with prio on lines
    SpriteSheet::concat(
        vec![three_sussyphuses.clone(), four_marios.clone()].into_iter(),
        Distribution::Packed(true),
    )
    .unwrap();

    // fixed columns
    SpriteSheet::concat(
        vec![three_sussyphuses.clone(), four_marios.clone()].into_iter(),
        Distribution::FixedColumns(4),
    )
    .unwrap();

    // fixed lines
    SpriteSheet::concat(
        vec![three_sussyphuses, four_marios].into_iter(),
        Distribution::FixedLines(1),
    )
    .unwrap();
}
