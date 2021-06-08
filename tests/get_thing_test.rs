#[test]
fn get_galaxy_trucker() -> boardgamegeek::Result<()> {
  let client = boardgamegeek::Client::new();

  let thing = tokio_test::block_on(client.get_thing("31481"))?;

  assert_eq!(thing.id, 31481);
  assert_eq!(thing.primary_name, "Galaxy Trucker");

  Ok(())
}
