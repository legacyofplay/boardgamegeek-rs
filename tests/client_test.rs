#[test]
fn get_collection() -> boardgamegeek::Result<()> {
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

#[test]
fn get_galaxy_trucker() -> boardgamegeek::Result<()> {
    let client = boardgamegeek::Client::new();

    let thing = tokio_test::block_on(client.get_thing("31481"))?;

    assert_eq!(thing.id, 31481);
    assert_eq!(thing.primary_name, "Galaxy Trucker");

    Ok(())
}

#[test]
fn get_random_boardgame_id() -> boardgamegeek::Result<()> {
    let client = boardgamegeek::Client::new();

    let id = tokio_test::block_on(client.get_random_boardgame_id())?;

    assert!(id.len() > 0);

    Ok(())
}

#[test]
fn get_top_expansions() -> boardgamegeek::Result<()> {
    let client = boardgamegeek::Client::new();

    let ids = tokio_test::block_on(client.get_top_expansions(0))?;

    assert!(ids.len() > 0);

    Ok(())
}

#[test]
fn get_top_games() -> boardgamegeek::Result<()> {
    let client = boardgamegeek::Client::new();

    let ids = tokio_test::block_on(client.get_top_games(0))?;

    assert!(ids.len() > 0);

    Ok(())
}
