import gleam/int
import gleam/io
import gleam/list
import gleam/order.{type Order, Eq, Gt, Lt}
import gleam/regex
import gleam/result
import gleam/string
import simplifile

type OneOrMany {
  One(Int)
  Many(List(OneOrMany))
}

type ListPair {
  ListPair(left: List(OneOrMany), right: List(OneOrMany))
}

// fn oom_to_list(oom: OneOrMany) -> String {
//   case oom {
//     One(int) -> int.to_string(int)
//     Many(list) -> list_to_string(list)
//   }
// }

// fn list_to_string(list: List(OneOrMany)) {
//   string.concat(["[", string.join(list.map(list, oom_to_list), ","), "]"])
// }

// fn pair_to_string(pair: ListPair) -> String {
//   string.concat([list_to_string(pair.left), "\n", list_to_string(pair.right)])
// }

// fn ord_to_string(ord: Order) -> String {
//   case ord {
//     Lt -> "Lt"
//     Eq -> "Eq"
//     Gt -> "Gt"
//   }
// }

fn parse_list(raw: String) -> List(OneOrMany) {
  let assert Ok(rx) = regex.from_string("\\[(?:[^\\[\\]]+|(?R))*+\\]|\\d+")
  let raw =
    raw
    |> string.drop_left(1)
    |> string.drop_right(1)
    |> regex.scan(rx, _)
    |> list.map(fn(match) { match.content })
  use raw_elem <- list.map(raw)

  case string.starts_with(raw_elem, "[") {
    True -> Many(parse_list(raw_elem))
    False -> {
      let assert Ok(int_) = int.parse(raw_elem)
      One(int_)
    }
  }
}

fn parse(raw: String) -> List(ListPair) {
  use chunk <- list.map(raw |> string.split("\n") |> list.sized_chunk(3))
  let assert [left_raw, right_raw, ""] = chunk

  ListPair(parse_list(left_raw), parse_list(right_raw))
}

fn compare_many(left: List(OneOrMany), right: List(OneOrMany)) -> Order {
  case left, right {
    [l_one, ..l_rest], [r_one, ..r_rest] -> {
      case compare(l_one, r_one) {
        Lt -> Lt
        Eq -> compare_many(l_rest, r_rest)
        Gt -> Gt
      }
    }
    _, _ -> int.compare(list.length(left), list.length(right))
  }
}

fn compare(left: OneOrMany, right: OneOrMany) -> Order {
  case left, right {
    One(left), One(right) -> int.compare(left, right)
    One(_), Many(_) -> compare(Many([left]), right)
    Many(_), One(_) -> compare(left, Many([right]))
    Many(left), Many(right) -> compare_many(left, right)
  }
}

fn part_one(input: List(ListPair)) -> Int {
  input
  |> list.index_map(fn(pair, i) {
    let res = #(i, compare(Many(pair.left), Many(pair.right)))
    res
  })
  |> list.filter(fn(pair) { pair.1 != Gt })
  |> list.map(fn(pair) { pair.0 })
  |> list.map(fn(x) { x + 1 })
  |> int.sum
}

pub fn main() {
  let assert Ok(input) = simplifile.read("../input/day13") |> result.map(parse)
  io.print("Part one: ")
  io.debug(part_one(input))
  Nil
}
