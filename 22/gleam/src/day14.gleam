import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import glearray.{type Array}
import simplifile

type Point {
  Point(x: Int, y: Int)
}

type Path =
  List(Point)

type Segment {
  Segment(start: Point, end: Point)
}

type Map =
  Set(Point)

type State {
  State(map: Map, depth: Int, start: Point, current: Point, sand_poured: Int)
}

fn parse(raw: String) -> List(Path) {
  let lines =
    raw |> string.split("\n") |> list.take_while(fn(s) { !string.is_empty(s) })
  use line <- list.map(lines)
  use raw_point <- list.map(line |> string.split(" -> "))

  let assert Ok(#(x_raw, y_raw)) = string.split_once(raw_point, ",")
  let assert #(Ok(x), Ok(y)) = #(int.parse(x_raw), int.parse(y_raw))

  Point(x:, y:)
}

fn segment_coords(segment: Segment) -> List(Point) {
  let is_horizontal = segment.start.x != segment.end.x
  case is_horizontal {
    True -> {
      let y = segment.start.y
      list.range(segment.start.x, segment.end.x)
      |> list.map(fn(x) { Point(x:, y:) })
    }
    False -> {
      let x = segment.start.x
      list.range(segment.start.y, segment.end.y)
      |> list.map(fn(y) { Point(x:, y:) })
    }
  }
}

fn add_path(map: Map, path: Path) -> Map {
  let path_coords =
    path
    |> list.window_by_2
    |> list.map(fn(pair) { Segment(start: pair.0, end: pair.1) })
    |> list.flat_map(segment_coords)

  set.union(map, set.from_list(path_coords))
}

fn paths_to_map(paths: List(Path)) -> Map {
  use acc, path <- list.fold(paths, set.new())
  add_path(acc, path)
}

fn get_depth(input: List(Path)) -> Int {
  use acc, point <- list.fold(input |> list.flatten, 0)
  int.max(acc, point.y)
}

fn down(point: Point) -> Point {
  Point(..point, y: point.y + 1)
}

fn left(point: Point) -> Point {
  Point(point.x - 1, point.y + 1)
}

fn right(point: Point) -> Point {
  Point(point.x + 1, point.y + 1)
}

fn pour_sand_to_overflow(state: State) -> Int {
  case state.current.y > state.depth {
    True -> state.sand_poured
    False ->
      case
        set.contains(state.map, down(state.current)),
        set.contains(state.map, left(state.current)),
        set.contains(state.map, right(state.current))
      {
        False, _, _ ->
          pour_sand_to_overflow(State(..state, current: down(state.current)))
        True, False, _ ->
          pour_sand_to_overflow(State(..state, current: left(state.current)))
        True, True, False ->
          pour_sand_to_overflow(State(..state, current: right(state.current)))
        _, _, _ ->
          pour_sand_to_overflow(
            State(
              ..state,
              map: set.insert(state.map, state.current),
              current: state.start,
              sand_poured: state.sand_poured + 1,
            ),
          )
      }
  }
}

fn part_one(input: List(Path)) -> Int {
  let #(map, depth) = #(paths_to_map(input), get_depth(input))
  let start = Point(500, 0)

  pour_sand_to_overflow(State(map, depth, start, start, 0))
}

pub fn main() {
  let assert Ok(input) = simplifile.read("../input/day14") |> result.map(parse)
  io.print("Part one: ")
  io.debug(part_one(input))
  // io.print("Part two: ")
  // io.debug(part_two(input))
  Nil
}
