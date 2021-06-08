#[test]
fn get_galaxy_trucker() -> boardgamegeek::Result<()> {
  let client = boardgamegeek::Client::new();

  let collection = tokio_test::block_on(
    client.get_collection("Schoonology", boardgamegeek::CollectionType::BoardGames),
  )?;

  let mut matches = 0;
  for item in collection.items {
    if item.id == 31481 {
      matches += 1;
    }
  }

  assert_eq!(matches, 1);

  Ok(())
}
