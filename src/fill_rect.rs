use super::range_map;

#[derive(Debug, Clone, PartialEq)]
pub struct FillRect {
    map: range_map::RangeMap<
        f64,
        Option<
            range_map::RangeMap<
                f64, Option<bool>
            >
        >
    >
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { b } else { a }
}
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

impl From<(f64, f64, f64, f64)> for FillRect {
    fn from(params: (f64, f64, f64, f64)) -> Self {
        FillRect::from(params.0, params.1, params.2, params.3)
    }
}

impl FillRect {
    pub fn from(
        min_x: f64, min_y: f64, max_x: f64, max_y: f64
    ) -> Self {
        let mut range_map = range_map::RangeMap::from(None);
        {
            let mut vertical_map = range_map::RangeMap::from(None);
            vertical_map.set(Some(false), min_y, max_y);
            range_map.set(Some(vertical_map), min_x, max_x);
        }
        Self {
            map: range_map
        }
    }

    pub fn is_fill(&self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> bool {
        for maps in self.map.values(min_x, max_x) {
            let Some(maps) = maps else { return false; };
            if let Some(false) = maps.value_range(min_y, max_y) {
                return false;
            }
        }

        return true;
    }

    pub fn fill_rect(&mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        if max_x < min_x {
            return self.fill_rect(max_x, min_y, min_x, max_y);
        }
        if max_y < min_y {
            return self.fill_rect(min_x, max_y, max_x, min_y);
        }

        for (vertical_ranges, local_min_x, local_max_x) in self.map.data_range(min_x, max_x) {
            let Some(mut vertical_ranges) = vertical_ranges else {
                continue
            };
            vertical_ranges.set(Some(true), min_y, max_y);
            self.map.set(
                Some(vertical_ranges),
                max(local_min_x, min_x),
                min(local_max_x, max_x),
            );
        }
    }

    pub fn get_open_rects(&self) -> Vec<(f64, f64, f64, f64)> {
        let mut r = Vec::new();

        for (range_map, min_x, max_x) in self.map.data() {
            let Some(range_map) = range_map else { continue };

            for (value, min_y, max_y) in range_map.data() {
                let Some(value) = value else { continue };
                if value || (max_x - min_x) * (max_y - min_y) <= 0.0 { continue; }

                r.push((min_x, min_y, max_x, max_y));
            }
        }

        return r;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn basic() {
        let fill_rect = FillRect::from(10.0, 100.0, 20.0, 110.0);
        assert_eq!(fill_rect.is_fill(10.0, 100.0, 20.0, 110.0), false);
    }

    #[test]
    pub fn basic_fill_rect() {
        let mut fill_rect = FillRect::from(10.0, 100.0, 20.0, 110.0);
        fill_rect.fill_rect(14.0, 104.0, 16.0, 106.0);
        assert_eq!(fill_rect.is_fill(14.0, 104.0, 16.0, 106.0), true);
    }

    #[test]
    pub fn fill_rect() {
        let mut fill_rect = FillRect::from(10.0, 100.0, 20.0, 110.0);
        for x in vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0] {
            fill_rect.fill_rect(x, 104.0, x + 1.0, 106.0);
        }
        assert_eq!(fill_rect.is_fill(10.0, 20.0, 16.0, 106.0), true);
    }

    #[test]
    pub fn get_open_rects() {
        let mut fill_rect = FillRect::from(10.0, 100.0, 20.0, 110.0);
        for x in vec![10.0, 11.0, 12.0, 13.0, 14.0] {
            fill_rect.fill_rect(x, 100.0, x + 1.0, 110.0);
        }
        assert_eq!(fill_rect.get_open_rects(), vec![(15.0, 100.0, 20.0, 110.0)]);
    }
}

