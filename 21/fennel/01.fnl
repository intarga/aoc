(local {:each f-each : zip : drop_n : reduce : head : tail} (require :fun))
(local input (icollect [line (io.lines "../INPUT/01")] (tonumber line)))

(fn count-inc [acc prev curr]
  (if (> curr prev) (+ acc 1) acc))

; part1
(reduce count-inc 0 (zip input (drop_n 1 input))) ; 1451

; part2
(reduce count-inc 0 (zip input (drop_n 3 input))) ; 1395
