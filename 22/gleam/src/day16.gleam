import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/option.{Some}
import gleam/regex
import gleam/result
import gleam/set.{type Set}
import gleam/string
import simplifile

type Link {
  Link(len: Int, end: String)
}

type Valve {
  Valve(name: String, flow: Int, links: List(Link))
}

type State {
  State(loc: Valve, opened: Set(String), time: Int, rate: Int, total: Int)
}

type SimRes {
  Continue(List(State))
  Done(Int)
}

fn parse(raw: String) -> List(Valve) {
  let options = regex.Options(case_insensitive: False, multi_line: True)
  let assert Ok(re) =
    regex.compile("^.*?([A-Z]{2}).*?(\\d+).*?valves?\\s(.*)$", options)
  use match <- list.map(regex.scan(re, raw))

  let assert [Some(name), Some(flow_raw), Some(links_raw)] = match.submatches

  let assert Ok(flow) = int.parse(flow_raw)
  let links = links_raw |> string.split(", ") |> list.map(fn(s) { Link(1, s) })

  Valve(name:, flow:, links:)
}

fn find_link(
  curr: List(String),
  visited: Set(String),
  end: String,
  valves: Dict(String, Valve),
  length: Int,
) -> Link {
  use <- bool.guard(list.contains(curr, end), Link(length, end))
  let nexts =
    curr
    |> list.flat_map(fn(c) {
      let assert Ok(v) = dict.get(valves, c)
      list.map(v.links, fn(l) { l.end })
    })
    |> list.filter(fn(n) { !set.contains(visited, n) })

  let visited = set.union(visited, set.from_list(nexts))
  find_link(nexts, visited, end, valves, length + 1)
}

fn simplify_valves(in: Dict(String, Valve)) -> Dict(String, Valve) {
  let working_valves =
    dict.filter(in, fn(_, v) { v.flow > 0 || v.name == "AA" })
  use k, v <- dict.map_values(working_valves)

  let links =
    dict.values(working_valves)
    |> list.filter_map(fn(v) {
      bool.guard(v.name == k, Error(Nil), fn() { Ok(v.name) })
    })
    |> list.map(find_link([k], set.from_list([k]), _, in, 0))
  Valve(..v, links:)
}

fn iter_state(state: State, valves: Dict(String, Valve)) -> SimRes {
  let time_left = 30 - state.time
  let nexts =
    list.filter(state.loc.links, fn(link) {
      !set.contains(state.opened, link.end) && link.len + 1 <= time_left
    })
  use <- bool.guard(
    list.is_empty(nexts),
    Done(state.total + state.rate * time_left),
  )

  Continue(
    nexts
    |> list.map(fn(next) {
      let assert Ok(valve) = dict.get(valves, next.end)
      State(
        loc: valve,
        opened: set.insert(state.opened, next.end),
        rate: state.rate + valve.flow,
        time: state.time + next.len + 1,
        total: state.total + state.rate * { next.len + 1 },
      )
    }),
  )
}

fn sim_all(
  states: List(State),
  endings: List(Int),
  valves: Dict(String, Valve),
) -> List(Int) {
  case states {
    [first, ..rest] ->
      case iter_state(first, valves) {
        Continue(states) -> sim_all(list.append(states, rest), endings, valves)
        Done(end) -> sim_all(rest, [end, ..endings], valves)
      }
    [] -> endings
  }
}

fn part_one(input: List(Valve)) -> Int {
  let valves =
    input
    |> list.map(fn(v) { #(v.name, v) })
    |> dict.from_list
    |> simplify_valves
  let assert Ok(start) = dict.get(valves, "AA")
  let end_states = sim_all([State(start, set.new(), 0, 0, 0)], [], valves)
  list.fold(end_states, 0, fn(acc, s) { int.max(acc, s) })
}

// fn part_two(input: List(Valve)) -> Int {
//   todo
// }

pub fn main() {
  let assert Ok(input) = simplifile.read("../input/day16") |> result.map(parse)
  //   let input =
  //     "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
  // Valve BB has flow rate=13; tunnels lead to valves CC, AA
  // Valve CC has flow rate=2; tunnels lead to valves DD, BB
  // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
  // Valve EE has flow rate=3; tunnels lead to valves FF, DD
  // Valve FF has flow rate=0; tunnels lead to valves EE, GG
  // Valve GG has flow rate=0; tunnels lead to valves FF, HH
  // Valve HH has flow rate=22; tunnel leads to valve GG
  // Valve II has flow rate=0; tunnels lead to valves AA, JJ
  // Valve JJ has flow rate=21; tunnel leads to valve II
  // "
  //     |> parse
  io.print("Part one: ")
  io.debug(part_one(input))
  // io.print("Part two: ")
  // io.debug(part_two(input))
  Nil
}
