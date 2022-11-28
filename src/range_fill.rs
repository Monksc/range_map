use algorithms;

#[derive(Debug, Clone, PartialEq)]
pub struct RangeFill {
    fill: Vec<f64>
}

impl From<(f64, f64)> for RangeFill {
    fn from(open_range: (f64, f64)) -> Self {
        Self {
            fill: vec![
                open_range.0,
                open_range.1,
            ]
        }
    }
}

impl RangeFill {
    pub fn from(min: f64, max: f64) -> Self {
        From::from((min, max))
    }
    pub fn fill(&mut self, min: f64, max: f64) {
        let mut seen_before_min = algorithms::seen_before_or_equal(&self.fill, min);
        let mut seen_before_max = algorithms::seen_before(&self.fill, max);

        if seen_before_min == seen_before_max {
            if seen_before_min % 2 == 1 {
                self.fill.insert(seen_before_min, max);
                self.fill.insert(seen_before_min, min);
            }
            return;
        }

        // is fill
        if seen_before_min % 2 == 1 {
            self.fill[seen_before_min] = min;
            seen_before_min += 1;
        }

        if seen_before_max % 2 == 1 {
            seen_before_max -= 1;
            self.fill[seen_before_max] = max;
        }

        self.fill.drain(seen_before_min..seen_before_max);
    }

    // [min, max)
    pub fn contains_unfilled(&self, min: f64, max: f64) -> bool {
        let seen_before_min = algorithms::seen_before_or_equal(&self.fill, min);
        let seen_before_max = algorithms::seen_before(&self.fill, max);

        !(seen_before_min % 2 == 0 && seen_before_min == seen_before_max)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_basic() {
        let range = RangeFill::from(10.0, 20.0);
        assert_eq!(range.contains_unfilled(-1.0, 9.99), false);
        assert_eq!(range.contains_unfilled(-1.0, 10.0), false);
        assert_eq!(range.contains_unfilled(10.0, 20.0), true);
        assert_eq!(range.contains_unfilled(15.0, 16.0), true);
        assert_eq!(range.contains_unfilled(20.0, 25.0), true);
        assert_eq!(range.contains_unfilled(21.0, 25.0), false);
    }

    #[test]
    pub fn test_simple_fill() {
        let mut range = RangeFill::from(10.0, 20.0);

        range.fill(11.0, 12.0);
        assert_eq!(range.contains_unfilled(11.0, 12.0), false);

        range.fill(13.0, 14.0);
        assert_eq!(range.contains_unfilled(13.0, 14.0), false);
        assert_eq!(range.fill, vec![10.0, 11.0, 12.0, 13.0, 14.0, 20.0]);

        range.fill(15.0, 16.0);
        assert_eq!(range.contains_unfilled(15.0, 16.0), false);

        range.fill(17.0, 18.0);
        assert_eq!(range.contains_unfilled(17.0, 18.0), false);
        assert_eq!(range.fill, vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 20.0]);
    }

    #[test]
    pub fn test_multiple_fills() {
        let mut range = RangeFill::from(10.0, 20.0);

        range.fill(11.0, 12.0);
        range.fill(13.0, 14.0);
        range.fill(15.0, 16.0);
        range.fill(17.0, 18.0);
        assert_eq!(range.fill, vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 20.0]);

        // Fill [Open, Close]
        range.fill(10.5, 15.5);
        assert_eq!(range.fill, vec![10.0, 10.5, 16.0, 17.0, 18.0, 20.0]);

        // Fill [Close, Open]
        range.fill(15.0, 16.5);
        assert_eq!(range.fill, vec![10.0, 10.5, 16.5, 17.0, 18.0, 20.0]);

        // Fill [Open, Open]
        range.fill(16.9, 18.5);
        assert_eq!(range.fill, vec![10.0, 10.5, 16.5, 16.9, 18.5, 20.0]);

        // Fill [Close, Close]
        range.fill(12.0, 18.0);
        assert_eq!(range.fill, vec![10.0, 10.5, 18.5, 20.0]);

        // Fill [Before, Close]
        range.fill(0.0, 18.0);
        assert_eq!(range.fill, vec![18.5, 20.0]);

        // Fill [Before, After]
        range.fill(0.0, 25.0);
        assert_eq!(range.fill, vec![]);
    }
}

