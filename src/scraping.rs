use itertools::Itertools;

pub fn id_from_url(url: &String) -> String {
  String::from(
    regex::Regex::new(r#"/boardgame/(\d+)/"#)
      .unwrap()
      .captures_iter(url)
      .map(|captures| captures.get(1).map_or("", |m| m.as_str()))
      .collect::<Vec<&str>>()[0],
  )
}

pub fn parse_game_ids_from_page(html: String) -> Vec<String> {
  regex::Regex::new(r#"href="/boardgame/(\d+)/"#)
    .unwrap()
    .captures_iter(html.as_str())
    .map(|captures| captures.get(1).map_or("", |m| m.as_str()))
    .sorted()
    .unique()
    .map(|s| String::from(s))
    .collect()
}

pub fn parse_expansion_ids_from_page(html: String) -> Vec<String> {
  regex::Regex::new(r#"href="/boardgameexpansion/(\d+)/"#)
    .unwrap()
    .captures_iter(html.as_str())
    .map(|captures| captures.get(1).map_or("", |m| m.as_str()))
    .sorted()
    .unique()
    .map(|s| String::from(s))
    .collect()
}
