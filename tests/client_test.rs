// #[test]
// fn get_collection() -> boardgamegeek::Result<()> {
//     let client = boardgamegeek::Client::new();

//     let collection = tokio_test::block_on(
//         client.get_collection("Schoonology", boardgamegeek::CollectionType::BoardGames),
//     )?;

//     let mut matches = 0;
//     for item in collection.items {
//         if item.id == 31481 {
//             matches += 1;
//         }
//     }

//     assert_eq!(matches, 1);

//     Ok(())
// }

#[test]
fn get_galaxy_trucker() -> boardgamegeek::Result<()> {
    let client = boardgamegeek::Client::new();

    let thing = tokio_test::block_on(client.get_boardgame("31481"))?;

    println!("Thing: {:?}", thing);

    assert_eq!(thing.id, 31481);
    assert_eq!(thing.primary_name, "Galaxy Trucker");

    Ok(())
}

#[test]
fn get_galaxy_trucker_legacy() -> boardgamegeek::Result<()> {
    let client = boardgamegeek::Client::new();

    let thing = tokio_test::block_on(client.get_thing("31481"))?;

    assert_eq!(thing.id, 31481);
    assert_eq!(thing.primary_name, "Galaxy Trucker");

    Ok(())
}

#[test]
fn get_missing_legacy() -> boardgamegeek::Result<()> {
    let client = boardgamegeek::Client::new();

    let thing = tokio_test::block_on(client.get_thing("337637"))?;

    assert_eq!(thing.id, 0);

    Ok(())
}
