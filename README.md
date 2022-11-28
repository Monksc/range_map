# range_map
A rust library that will map a range of values to another value.


```
...
[dependencies]
range_map = {  git = "https://github.com/Monksc/range_map", rev = "ef2d106"}
```

# Example
```
let mut range = range_map::RangeMap::from(None);
range.set(Some(false), 10.0, 20.0);

range.set(Some(true), 11.0, 12.0);
range.set(Some(true), 13.0, 14.0);
range.set(Some(true), 15.0, 16.0);
range.set(Some(true), 17.0, 18.0);

assert_eq!(range.value_range(17.0, 18.0), &Some(true));
assert_eq!(range.value_range(10.0, 20.0), &None);
assert_eq!(range.value_range(12.0, 13.0), &Some(false));
assert_eq!(range.value(11.5), &Some(true));
assert_eq!(range.values(10.0, 20.0), vec![
    &Some(false),
    &Some(true),
    &Some(false),
    &Some(true),
    &Some(false),
    &Some(true),
    &Some(false),
    &Some(true),
    &Some(false),
]);
assert_eq!(range.values(14.5, 15.5), vec![
    &Some(false),
    &Some(true),
]);
```

```
#[derive(Debug, Clone, PartialEq)]
enum Quadrant {
    Off,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

let mut left_side = range_map::RangeMap::from(Quadrant::Off);
left_side.set(Quadrant::UpperLeft, 0.0, 1.0);
left_side.set(Quadrant::LowerLeft, -1.0, 0.0);

let mut right_side = range_map::RangeMap::from(Quadrant::Off);
right_side.set(Quadrant::UpperRight, 0.0, 1.0);
right_side.set(Quadrant::LowerRight, -1.0, 0.0);

let mut quads = range_map::RangeMap::from(None);
quads.set(left_side, -1.0, 0.0);
quads.set(right_side, 0.0, 1.0);

assert_eq!(quads.value(-0.5), &Some(left_side));
assert_eq!(quads.value(0.5), &Some(right_side));
assert_eq!(quads.value(1.5), &None);
assert_eq!(quads.value(-1.5), &None);

assert_eq!(left_side.value(0.5), &Quadrant::UpperLeft);
assert_eq!(left_side.value(-0.5), &Quadrant::LowerLeft);
assert_eq!(left_side.value(1.5), &Quadrant::Off);
assert_eq!(left_side.value(-1.5), &Quadrant::Off);

assert_eq!(right_side.value(0.5), &Quadrant::UpperRight);
assert_eq!(right_side.value(-0.5), &Quadrant::LowerRight);
assert_eq!(right_side.value(1.5), &Quadrant::Off);
assert_eq!(right_side.value(-1.5), &Quadrant::Off);
```
