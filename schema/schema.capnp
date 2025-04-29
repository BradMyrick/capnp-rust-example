@0xdd0a16e0a7f65571;
struct Point {
  x @0 :Float32;
  y @1 :Float32;
}

interface PointTracker {
  addPoint @0 (p :Point) -> (totalPoints :UInt64);
}

