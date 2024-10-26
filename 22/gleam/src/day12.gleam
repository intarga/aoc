import gleam/io
import gleam/iterator.{type Iterator}
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import glearray.{type Array}
import simplifile

type Point {
  Point(x: Int, y: Int)
}

type Map {
  Map(vals: Array(Array(String)), dims: Point)
}

fn char_height(char: String) -> Int {
  case char {
    "S" -> 1
    "E" -> 26
    _ -> {
      let assert Ok(codepoint) = string.to_utf_codepoints(char) |> list.first
      string.utf_codepoint_to_int(codepoint) - 96
    }
  }
}

fn all_points(map: Map) -> Iterator(Point) {
  iterator.range(0, map.dims.x - 1)
  |> iterator.flat_map(fn(x) {
    iterator.repeat(x) |> iterator.zip(iterator.range(0, map.dims.y - 1))
  })
  |> iterator.map(fn(pair) { Point(pair.0, pair.1) })
}

fn at_point(map: Map, point: Point) -> String {
  let assert Ok(inner) = glearray.get(map.vals, point.x)
  let assert Ok(val) = glearray.get(inner, point.y)
  val
}

fn neighbours(map: Map, point: Point) -> List(Point) {
  let x = point.x
  let y = point.y
  [Point(x + 1, y), Point(x - 1, y), Point(x, y + 1), Point(x, y - 1)]
  |> list.filter_map(fn(neighbour) {
    let within_x = neighbour.x >= 0 && neighbour.x < map.dims.x
    let within_y = neighbour.y >= 0 && neighbour.y < map.dims.y
    let allowed_height = fn() {
      char_height(at_point(map, neighbour))
      <= char_height(at_point(map, point)) + 1
    }
    case within_x && within_y && allowed_height() {
      True -> Ok(neighbour)
      False -> Error(Nil)
    }
  })
}

fn find_start(map: Map) -> Point {
  case
    all_points(map) |> iterator.find(fn(point) { at_point(map, point) == "S" })
  {
    Ok(point) -> point
    Error(_) -> panic as "could not find start"
  }
}

fn find_lowest(map: Map) -> List(Point) {
  all_points(map)
  |> iterator.filter(fn(point) {
    at_point(map, point) == "a" || at_point(map, point) == "S"
  })
  |> iterator.to_list
}

fn explore(
  map: Map,
  points: List(Point),
  visited: Set(Point),
  steps: Int,
) -> Int {
  let neighbours =
    points
    |> list.flat_map(fn(point) { neighbours(map, point) })
    |> set.from_list
    |> set.filter(fn(point) { !set.contains(visited, point) })

  let visited = set.union(visited, neighbours)

  let neighbours = neighbours |> set.to_list

  let done = list.any(neighbours, fn(point) { at_point(map, point) == "E" })

  case done {
    True -> steps + 1
    False -> explore(map, neighbours, visited, steps + 1)
  }
}

fn parse(raw: String) -> Map {
  let vals =
    raw
    |> string.split("\n")
    |> list.take_while(fn(s) { !string.is_empty(s) })
    |> list.map(string.to_graphemes)
    |> list.map(glearray.from_list)
    |> glearray.from_list

  let dims =
    Point(
      glearray.length(vals),
      glearray.get(vals, 0)
        |> result.map(fn(inner) { glearray.length(inner) })
        |> result.unwrap(0),
    )

  Map(vals:, dims:)
}

fn part_one(input: Map) -> Int {
  let start = [find_start(input)]
  explore(input, start, set.from_list(start), 0)
}

fn part_two(input: Map) -> Int {
  let lowest = find_lowest(input)
  explore(input, lowest, set.from_list(lowest), 0)
}

pub fn main() {
  let assert Ok(input) = simplifile.read("../input/day12") |> result.map(parse)
  io.print("Part one: ")
  io.debug(part_one(input))
  io.print("Part two: ")
  io.debug(part_two(input))
}
