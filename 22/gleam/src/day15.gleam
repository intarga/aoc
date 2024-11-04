import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/regex
import gleam/result
import simplifile

type Point {
  Point(x: Int, y: Int)
}

type Sensor {
  Sensor(loc: Point, beacon: Point)
}

type Range {
  Range(from: Int, to: Int)
}

fn parse(raw: String) -> List(Sensor) {
  let assert Ok(re) = regex.from_string("-?\\d+")
  use coords <- list.map(
    regex.scan(re, raw)
    |> list.filter_map(fn(m) { int.parse(m.content) })
    |> list.sized_chunk(4),
  )
  let assert [s_x, s_y, b_x, b_y] = coords

  Sensor(Point(s_x, s_y), Point(b_x, b_y))
}

fn sensor_checked_in_row(sensor: Sensor, row: Int) -> Result(Range, Nil) {
  let manhattan_distance =
    int.absolute_value(sensor.loc.x - sensor.beacon.x)
    + int.absolute_value(sensor.loc.y - sensor.beacon.y)
  let row_distance = int.absolute_value(sensor.loc.y - row)

  let spare_distance = manhattan_distance - row_distance

  case spare_distance >= 0 {
    True ->
      Ok(Range(sensor.loc.x - spare_distance, sensor.loc.x + spare_distance))
    False -> Error(Nil)
  }
}

fn merge_ranges(ranges: List(Range)) -> List(Range) {
  case ranges {
    [r1, r2, ..rest] ->
      case r1.to + 1 >= r2.from {
        True -> merge_ranges([Range(r1.from, int.max(r1.to, r2.to)), ..rest])
        False -> [r1, ..merge_ranges([r2, ..rest])]
      }
    list -> list
  }
}

fn checked_ranges(sensors: List(Sensor), row: Int) -> List(Range) {
  list.filter_map(sensors, sensor_checked_in_row(_, row))
  |> list.sort(fn(r1, r2) { int.compare(r1.from, r2.from) })
  |> merge_ranges
}

fn part_one(input: List(Sensor)) -> Int {
  let row = 2_000_000

  let checked =
    checked_ranges(input, row)
    |> list.map(fn(range) { 1 + range.to - range.from })
    |> int.sum

  let beacons_in_row =
    input
    |> list.filter_map(fn(sensor) {
      bool.guard(sensor.beacon.y == row, Ok(sensor.beacon), fn() { Error(Nil) })
    })
    |> list.unique
    |> list.length

  checked - beacons_in_row
}

fn available_in_row(sensors: List(Sensor), row: Int) -> Result(Point, Nil) {
  case checked_ranges(sensors, row) {
    [r1, _] -> Ok(Point(r1.to + 1, row))
    _ -> Error(Nil)
  }
}

fn part_two(input: List(Sensor)) -> Int {
  let assert Ok(point) =
    list.range(0, 4_000_000)
    |> list.find_map(available_in_row(input, _))
  point.x * 4_000_000 + point.y
}

pub fn main() {
  let assert Ok(input) = simplifile.read("../input/day15") |> result.map(parse)
  io.print("Part one: ")
  io.debug(part_one(input))
  io.print("Part two: ")
  io.debug(part_two(input))
  Nil
}
