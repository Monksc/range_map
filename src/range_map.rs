#[derive(Debug, Clone, PartialEq)]
pub struct RangeMap<K: PartialOrd + Copy, V: PartialEq + Clone> {
    default_value: V,
    values: Vec<V>,
    ranges: Vec<K>,
}

impl<K: PartialOrd + Copy, V: PartialEq + Clone> From<V> for RangeMap<K, V> {
    fn from(out_of_range_value: V) -> Self {
        Self {
            default_value: out_of_range_value,
            values: Vec::new(),
            ranges: Vec::new(),
        }
    }
}

impl<K: PartialOrd + Copy, V: PartialEq + Clone> RangeMap<K, V> {
    pub fn from(out_of_range_value: V) -> Self {
        From::from(out_of_range_value)
    }

    pub fn data(&self) -> Vec<(&V, K, K)> {
        let mut r = Vec::new();

        for i in 0..self.values.len() {
            r.push((&self.values[i], self.ranges[i], self.ranges[i+1]));
        }

        return r;
    }

    pub fn default_value(&self) -> &V {
        &self.default_value
    }

    pub fn set(&mut self, value: V, min: K, max: K) {
        let mut min = min;
        let mut max = max;

        let mut seen_before_min = algorithms::seen_before_address(&self.ranges, &min);
        let mut seen_before_max = algorithms::seen_before_or_equal_address(&self.ranges, &max);

        if seen_before_min > 0 &&
            self.values[seen_before_min-1] == value &&
            min != self.ranges[seen_before_min-1] {
            min = self.ranges[seen_before_min-1];
            seen_before_min -= 1;
        }

        if seen_before_max <= self.values.len() &&
            ((seen_before_max > 0 && self.values[seen_before_max-1] == value) ||
             (seen_before_max == 0 && self.default_value == value)) &&
            max != self.ranges[seen_before_max] {
            max = self.ranges[seen_before_max];
            seen_before_max += 1;
        }

        let mut should_replace = true;
        let replace_value = self.value_address(&max).clone();
        if seen_before_max > seen_before_min {
            if seen_before_max > self.values.len() {
                self.ranges.drain(seen_before_min..seen_before_max);
                self.values.drain(seen_before_min..seen_before_max-1);
                should_replace = false;
            } else {
                self.ranges.drain(seen_before_min..seen_before_max);
                self.values.drain(seen_before_min..seen_before_max);
            }
        }

        if self.ranges.len() > 0 && should_replace {
            self.values.insert(seen_before_min, replace_value);
        }
        self.values.insert(seen_before_min, value);

        self.ranges.insert(seen_before_min, max);
        self.ranges.insert(seen_before_min, min);

        // prune outsides
        if self.values.len() > 0 && self.values[0] == self.default_value {
            self.values.drain(0..1);
            self.ranges.drain(0..1);
            if self.ranges.len() == 1 {
                self.ranges.drain(0..1);
            }
        }
        if self.values.len() > 0 && self.values[self.values.len()-1] == self.default_value {
            self.values.drain(self.values.len()-1..self.values.len());
            self.ranges.drain(self.ranges.len()-1..self.ranges.len());
            if self.ranges.len() == 1 {
                self.ranges.drain(0..1);
            }
        }
    }

    pub fn value(&self, value: K) -> &V {
        let seen_before = algorithms::seen_before_or_equal(&self.ranges, value);
        if seen_before == 0 {
            &self.default_value
        } else {
            &self.values[seen_before-1]
        }
    }

    pub fn value_address(&self, value: &K) -> &V {
        let seen_before = algorithms::seen_before_or_equal_address(&self.ranges, value);
        if seen_before == 0 || seen_before > self.values.len() {
            &self.default_value
        } else {
            &self.values[seen_before-1]
        }
    }

    pub fn value_range(&self, min: K, max: K) -> &V {
        let seen_before_min = algorithms::seen_before_or_equal(&self.ranges, min);
        let seen_before_max = algorithms::seen_before(&self.ranges, max);
        if seen_before_min != seen_before_max {
            &self.default_value
        } else if seen_before_min == 0 {
            &self.default_value
        } else if seen_before_min >= self.ranges.len() {
            &self.default_value
        } else {
            &self.values[seen_before_min-1]
        }
    }

    // [min, max)
    pub fn values(&self, min: K, max: K) -> Vec::<&V> {
        let mut seen_before_min = algorithms::seen_before_or_equal(&self.ranges, min);
        let mut seen_before_max = algorithms::seen_before(&self.ranges, max);

        if seen_before_min == 0 {
            seen_before_min = 1;
        }
        if seen_before_max-1>= self.values.len() {
            seen_before_max = self.values.len();
        }

        let mut r = Vec::new();
        for i in seen_before_min..=seen_before_max {
            r.push(&self.values[i-1]);
        }

        return r;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_basic() {
        let mut range = RangeMap::from(true);
        range.set(false, 10.0, 20.0);
        assert_eq!(range.ranges, vec![10.0, 20.0]);
        assert_eq!(range.values, vec![false]);
        assert_eq!(range.default_value, true);
        assert_eq!(range.value_range(-1.0, 9.99), &true);
        assert_eq!(range.value_range(-1.0, 10.0), &true);
        assert_eq!(range.value_range(10.0, 20.0), &false);
        assert_eq!(range.value_range(15.0, 16.0), &false);
        assert_eq!(range.value_range(20.0, 25.0), &true);
        assert_eq!(range.value_range(21.0, 25.0), &true);
    }

    #[test]
    pub fn test_simple_fill() {
        let mut range = RangeMap::from(true);
        range.set(false, 10.0, 20.0);
        range.set(false, 11.0, 12.0);
        assert_eq!(range.ranges, vec![10.0, 20.0]);
        assert_eq!(range.values, vec![false]);

        range.set(true, 11.0, 12.0);
        assert_eq!(range.values, vec![false, true, false]);
        assert_eq!(range.ranges, vec![10.0, 11.0, 12.0, 20.0]);
        assert_eq!(range.value_range(11.0, 12.0), &true);

        range.set(true, 13.0, 14.0);
        assert_eq!(range.values, vec![false, true, false, true, false]);
        assert_eq!(range.ranges, vec![10.0, 11.0, 12.0, 13.0, 14.0, 20.0]);
        assert_eq!(range.value_range(13.0, 14.0), &true);

        range.set(true, 15.0, 16.0);
        assert_eq!(range.value_range(15.0, 16.0), &true);

        range.set(true, 17.0, 18.0);
        assert_eq!(range.value_range(17.0, 18.0), &true);
        assert_eq!(range.ranges, vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 20.0]);
        assert_eq!(range.values, vec![false, true, false, true, false, true, false, true, false]);
    }

    #[test]
    pub fn test_multiple_fills() {
        let mut range = RangeMap::from(true);
        range.set(false, 10.0, 20.0);

        range.set(true, 11.0, 12.0);
        range.set(true, 13.0, 14.0);
        range.set(true, 15.0, 16.0);
        range.set(true, 17.0, 18.0);
        assert_eq!(range.ranges, vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 20.0]);
        assert_eq!(range.values, vec![false, true, false, true, false, true, false, true, false]);

        // Fill [Open, Close]
        range.set(true, 10.5, 15.5);
        assert_eq!(range.ranges, vec![10.0, 10.5, 16.0, 17.0, 18.0, 20.0]);
        assert_eq!(range.values, vec![false, true, false, true, false ]);

        // Fill [Close, Open]
        range.set(true, 15.0, 16.5);
        assert_eq!(range.ranges, vec![10.0, 10.5, 16.5, 17.0, 18.0, 20.0]);

        // Fill [Open, Open]
        range.set(true, 16.9, 18.5);
        assert_eq!(range.ranges, vec![10.0, 10.5, 16.5, 16.9, 18.5, 20.0]);

        // Fill [Close, Close]
        range.set(true, 12.0, 18.0);
        assert_eq!(range.ranges, vec![10.0, 10.5, 18.5, 20.0]);
        assert_eq!(range.values, vec![false, true, false]);

        // Fill [Before, Close]
        range.set(true, 0.0, 18.0);
        assert_eq!(range.ranges, vec![18.5, 20.0]);
        assert_eq!(range.values, vec![false]);

        // Fill [Before, After]
        range.set(true, 0.0, 25.0);
        assert_eq!(range.ranges, vec![]);
        assert_eq!(range.values, vec![]);
    }
}
