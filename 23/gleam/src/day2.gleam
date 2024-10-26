import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

const max_red = 12

const max_green = 13

const max_blue = 14

type Color {
  Red
  Green
  Blue
}

type Subset {
  Subset(count: Int, color: Color)
}

type Set {
  Set(reds: Int, greens: Int, blues: Int)
}

type Game {
  Game(id: Int, sets: List(Set))
}

fn parse_subset(raw: String) -> Subset {
  let assert Ok(#(count_raw, color_raw)) = string.split_once(raw, " ")

  let assert Ok(count) = int.parse(count_raw)
  let color = case color_raw {
    "red" -> Red
    "green" -> Green
    "blue" -> Blue
    _ -> panic as "invalid color"
  }

  Subset(count:, color:)
}

fn parse_set(raw: String) -> Set {
  raw
  |> string.split(", ")
  |> list.map(parse_subset)
  |> list.fold(Set(0, 0, 0), fn(set, subset) {
    case subset.color {
      Red -> Set(..set, reds: subset.count)
      Green -> Set(..set, greens: subset.count)
      Blue -> Set(..set, blues: subset.count)
    }
  })
}

fn parse_sets(raw: String) -> List(Set) {
  raw |> string.split("; ") |> list.map(parse_set)
}

fn parse_game(raw: String) -> Game {
  let assert Ok(#(id_raw, sets_raw)) = string.split_once(raw, ": ")
  let assert Ok(id) = int.parse(string.drop_left(id_raw, 5))

  Game(id:, sets: parse_sets(sets_raw))
}

fn parse_input(input: String) -> List(Game) {
  input |> string.split("\n") |> list.map(parse_game)
}

fn is_set_allowed(set: Set) -> Bool {
  set.reds <= max_red && set.greens <= max_green && set.blues <= max_blue
}

fn is_game_allowed(game: Game) -> Bool {
  list.all(game.sets, is_set_allowed)
}

fn part_one(input: List(Game)) -> Int {
  input
  |> list.filter(is_game_allowed)
  |> list.map(fn(game) { game.id })
  |> list.fold(0, fn(acc, x) { acc + x })
}

fn set_power(set: Set) -> Int {
  set.reds * set.greens * set.blues
}

fn part_two(input: List(Game)) -> Int {
  input
  |> list.map(fn(game) {
    game.sets
    |> list.fold(Set(0, 0, 0), fn(acc, x) {
      Set(
        int.max(acc.reds, x.reds),
        int.max(acc.greens, x.greens),
        int.max(acc.blues, x.blues),
      )
    })
    |> set_power
  })
  |> list.fold(0, fn(acc, x) { acc + x })
}

pub fn main() {
  let assert Ok(raw_input) = simplifile.read("../input/day2")
  let input = parse_input(raw_input)

  io.debug(#("Part 1:", part_one(input)))
  io.debug(#("Part 2:", part_two(input)))
}
