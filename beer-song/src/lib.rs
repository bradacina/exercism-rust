const TWO_BEER:&str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";
const ONE_BEER:&str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";
const NO_BEER:&str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";

fn many_beers(n: i32) -> String {
    format!(
        "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",
        n,
        n - 1
    )
}

fn choose_verse(n: i32) -> String {
    match n {
        0 => NO_BEER.to_string(),
        1 => ONE_BEER.to_string(),
        2 => TWO_BEER.to_string(),
        n => many_beers(n),
    }
}

pub fn verse(n: i32) -> String {
    choose_verse(n)
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(choose_verse)
        .collect::<Vec<_>>()
        .join("\n")
}
