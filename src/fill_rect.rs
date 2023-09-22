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

    pub fn is_fill_point(&mut self, x: f64, y: f64) -> bool {
        if let Some(y_range) = self.map.value(x) {
            if let Some(is_fill) = y_range.value(y) {
                return *is_fill;
            }
        }
        return true;
    }


    pub fn is_fill(&self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> bool {
        for maps in self.map.values(min_x, max_x) {
            let Some(maps) = maps else { return false; };
            for value in maps.values(min_y, max_y) {
                let Some(value) = value else { continue };
                if *value == false {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn is_fill_padding(
        &self,
        min_x: f64, min_y: f64,
        max_x: f64, max_y: f64,
        x_padding: f64, y_padding: f64
    ) -> bool {
        if max_x < min_x {
            return self.is_fill_padding(max_x, min_y, min_x, max_y, x_padding, y_padding);
        }
        if max_y < min_y {
            return self.is_fill_padding(min_x, max_y, max_x, min_y, x_padding, y_padding);
        }
        return self.is_fill(
            min_x - x_padding, min_y - y_padding,
            max_x + x_padding, max_y + y_padding
        );
    }

    pub fn biggest_unfilled_area_block(&self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> f64 {
        let mut biggest_area = 0.0;
        for (maps, local_min_x, local_max_x) in self.map.data_range_address(min_x, max_x) {
            let Some(maps) = maps else {
                let area = (local_max_x - local_min_x) * (max_y - min_y);
                if biggest_area < area {
                    biggest_area = area;
                }
                continue;
            };
            for (value, local_min_y, local_max_y) in maps.data_range_address(min_y, max_y) {
                let Some(value) = value else { continue };
                if *value == false {
                    let area = (local_max_x - local_min_x) * (local_max_y - local_min_y);
                    if biggest_area < area {
                        biggest_area = area;
                    }
                }
            }
        }
        return biggest_area;
    }

    pub fn biggest_unfilled_area_block_padding(
        &self,
        min_x: f64, min_y: f64,
        max_x: f64, max_y: f64,
        x_padding: f64, y_padding: f64
    ) -> f64 {
        if max_x < min_x {
            return self.biggest_unfilled_area_block_padding(max_x, min_y, min_x, max_y, x_padding, y_padding);
        }
        if max_y < min_y {
            return self.biggest_unfilled_area_block_padding(min_x, max_y, max_x, min_y, x_padding, y_padding);
        }
        return self.biggest_unfilled_area_block(
            min_x - x_padding, min_y - y_padding,
            max_x + x_padding, max_y + y_padding
        );
    }

    pub fn contains_unfilled_rect_width_height(
        &self,
        min_x: f64, min_y: f64,
        max_x: f64, max_y: f64,
        width: f64, height: f64
    ) -> bool {
        for (maps, local_min_x, local_max_x) in self.map.data_range_address(min_x, max_x) {
            let Some(maps) = maps else {
                if (local_max_x - local_min_x) > width &&
                    (max_y - min_y) > height
                {
                    return true;
                }
                continue;
            };
            for (value, local_min_y, local_max_y) in maps.data_range_address(min_y, max_y) {
                let Some(value) = value else { continue };
                if *value == false {
                    if (local_max_x - local_min_x) > width &&
                        (local_max_y - local_min_y) > height
                    {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn contains_unfilled_rect_width_height_padding(
        &self,
        min_x: f64, min_y: f64,
        max_x: f64, max_y: f64,
        width: f64, height: f64,
        x_padding: f64, y_padding: f64,
    ) -> bool {
        if max_x < min_x {
            return self.contains_unfilled_rect_width_height_padding(
                max_x, min_y, min_x, max_y, width, height, x_padding, y_padding,
            );
        }
        if max_y < min_y {
            return self.contains_unfilled_rect_width_height_padding(
                min_x, max_y, max_x, min_y, width, height, x_padding, y_padding,
            );
        }
        return self.contains_unfilled_rect_width_height(
            min_x - x_padding, min_y - y_padding,
            max_x + x_padding, max_y + y_padding,
            width, height,
        );
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

    pub fn get_ys(&self, x: f64) -> Option<&Vec<f64>> {
        if let Some(range) = self.map.value(x) {
            Some(range.get_ranges())
        } else {
            None
        }
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

    #[test]
    pub fn test_sliver_on_end() {
        let mut fill_rect = FillRect::from(10.0, 100.0, 20.0, 110.0);
        fill_rect.fill_rect(10.0, 100.0, 16.0, 110.0);
        fill_rect.fill_rect(20.0, 100.0, 20.0, 110.0);
        fill_rect.fill_rect(20.0, 105.0, 22.0, 110.0);
        assert_eq!(fill_rect.get_open_rects(), vec![(16.0, 100.0, 20.0, 110.0)]);
    }

    #[test]
    pub fn test_gcode_package_end_line_error() {
        let mut fill_rect = FillRect::from(1.0, 1.0, 7.0, 7.0);
        fill_rect.fill_rect(1.0, 1.0, 1.1, 7.0);
        fill_rect.fill_rect(1.0, 6.9, 1.2000000000000002, 7.0);
        fill_rect.fill_rect(1.1, 1.0, 1.2000000000000002, 7.0);
        fill_rect.fill_rect(1.1, 1.0, 1.3000000000000003, 1.1);
        fill_rect.fill_rect(1.2000000000000002, 1.0, 1.3000000000000003, 7.0);
        fill_rect.fill_rect(1.2000000000000002, 6.9, 1.4000000000000004, 7.0);
        fill_rect.fill_rect(1.3000000000000003, 1.0, 1.4000000000000004, 7.0);
        fill_rect.fill_rect(1.3000000000000003, 1.0, 1.5000000000000004, 1.1);
        fill_rect.fill_rect(1.4000000000000004, 1.0, 1.5000000000000004, 7.0);
        fill_rect.fill_rect(1.4000000000000004, 6.9, 1.6000000000000005, 7.0);
        fill_rect.fill_rect(1.5000000000000004, 1.0, 1.6000000000000005, 7.0);
        fill_rect.fill_rect(1.5000000000000004, 1.0, 1.7000000000000006, 1.1);
        fill_rect.fill_rect(1.6000000000000005, 1.0, 1.7000000000000006, 7.0);
        fill_rect.fill_rect(1.6000000000000005, 6.9, 1.8000000000000007, 7.0);
        fill_rect.fill_rect(1.7000000000000006, 1.0, 1.8000000000000007, 7.0);
        fill_rect.fill_rect(1.7000000000000006, 1.0, 1.9000000000000008, 1.1);
        fill_rect.fill_rect(1.8000000000000007, 1.0, 1.9000000000000008, 7.0);
        fill_rect.fill_rect(1.8000000000000007, 6.9, 2.000000000000001, 7.0);
        fill_rect.fill_rect(1.9000000000000008, 1.0, 2.000000000000001, 7.0);
        fill_rect.fill_rect(1.9000000000000008, 1.0, 2.1000000000000005, 1.1);
        fill_rect.fill_rect(2.000000000000001, 1.0, 2.1000000000000005, 3.9499999999999993);
        fill_rect.fill_rect(1.9000000000000008, 4.000000000000003, 2.1000000000000005, 4.100000000000002);
        fill_rect.fill_rect(2.000000000000001, 4.000000000000003, 2.1000000000000005, 7.0);
        fill_rect.fill_rect(2.000000000000001, 6.9, 2.2000000000000006, 7.0);
        fill_rect.fill_rect(2.100000000000001, 4.15, 2.2000000000000006, 7.0);
        fill_rect.fill_rect(2.000000000000001, 1.0, 2.2000000000000006, 1.1);
        fill_rect.fill_rect(2.100000000000001, 1.0, 2.2000000000000006, 3.849999999999999);
        fill_rect.fill_rect(2.100000000000001, 1.0, 2.3000000000000007, 1.1);
        fill_rect.fill_rect(2.200000000000001, 1.0, 2.3000000000000007, 3.749999999999999);
        fill_rect.fill_rect(2.100000000000001, 4.200000000000002, 2.3000000000000007, 4.300000000000002);
        fill_rect.fill_rect(2.200000000000001, 4.200000000000002, 2.3000000000000007, 7.0);
        fill_rect.fill_rect(2.200000000000001, 6.9, 2.400000000000001, 7.0);
        fill_rect.fill_rect(2.300000000000001, 4.350000000000001, 2.400000000000001, 7.0);
        fill_rect.fill_rect(2.200000000000001, 1.0, 2.400000000000001, 1.1);
        fill_rect.fill_rect(2.300000000000001, 1.0, 2.400000000000001, 3.649999999999999);
        fill_rect.fill_rect(2.300000000000001, 1.0, 2.500000000000001, 1.1);
        fill_rect.fill_rect(2.4000000000000012, 1.0, 2.500000000000001, 3.549999999999999);
        fill_rect.fill_rect(2.300000000000001, 4.500000000000001, 2.500000000000001, 4.6000000000000005);
        fill_rect.fill_rect(2.4000000000000012, 4.500000000000001, 2.500000000000001, 7.0);
        fill_rect.fill_rect(2.4000000000000012, 6.9, 2.600000000000001, 7.0);
        fill_rect.fill_rect(2.5000000000000013, 4.550000000000001, 2.600000000000001, 7.0);
        fill_rect.fill_rect(2.4000000000000012, 1.0, 2.600000000000001, 1.1);
        fill_rect.fill_rect(2.5000000000000013, 1.0, 2.600000000000001, 3.449999999999999);
        fill_rect.fill_rect(2.5000000000000013, 1.0, 2.700000000000001, 1.1);
        fill_rect.fill_rect(2.6000000000000014, 1.0, 2.700000000000001, 3.3499999999999988);
        fill_rect.fill_rect(2.5000000000000013, 4.7, 2.700000000000001, 4.8);
        fill_rect.fill_rect(2.6000000000000014, 4.7, 2.700000000000001, 7.0);
        fill_rect.fill_rect(2.6000000000000014, 6.9, 2.800000000000001, 7.0);
        fill_rect.fill_rect(2.7000000000000015, 4.750000000000002, 2.800000000000001, 7.0);
        fill_rect.fill_rect(2.6000000000000014, 1.0, 2.800000000000001, 1.1);
        fill_rect.fill_rect(2.7000000000000015, 1.0, 2.800000000000001, 3.2499999999999987);
        fill_rect.fill_rect(2.7000000000000015, 1.0, 2.9000000000000012, 1.1);
        fill_rect.fill_rect(2.8000000000000016, 1.0, 2.9000000000000012, 3.1499999999999986);
        fill_rect.fill_rect(2.7000000000000015, 4.8999999999999995, 2.9000000000000012, 4.999999999999999);
        fill_rect.fill_rect(2.8000000000000016, 4.8999999999999995, 2.9000000000000012, 7.0);
        fill_rect.fill_rect(2.8000000000000016, 6.9, 3.0000000000000013, 7.0);
        fill_rect.fill_rect(2.9000000000000017, 4.950000000000001, 3.0000000000000013, 7.0);
        fill_rect.fill_rect(2.8000000000000016, 1.0, 3.0000000000000013, 1.1);
        fill_rect.fill_rect(2.9000000000000017, 1.0, 3.0000000000000013, 3.0499999999999985);
        fill_rect.fill_rect(2.9000000000000017, 1.0, 3.1000000000000014, 1.1);
        fill_rect.fill_rect(3.0000000000000018, 1.0, 3.1000000000000014, 2.9499999999999984);
        fill_rect.fill_rect(2.9000000000000017, 5.099999999999999, 3.1000000000000014, 5.199999999999998);
        fill_rect.fill_rect(3.0000000000000018, 5.099999999999999, 3.1000000000000014, 7.0);
        fill_rect.fill_rect(3.0000000000000018, 6.9, 3.2000000000000015, 7.0);
        fill_rect.fill_rect(3.100000000000002, 5.150000000000002, 3.2000000000000015, 7.0);
        fill_rect.fill_rect(3.0000000000000018, 1.0, 3.2000000000000015, 1.1);
        fill_rect.fill_rect(3.100000000000002, 1.0, 3.2000000000000015, 2.8499999999999983);
        fill_rect.fill_rect(3.100000000000002, 1.0, 3.3000000000000016, 1.1);
        fill_rect.fill_rect(3.200000000000002, 1.0, 3.3000000000000016, 2.7499999999999982);
        fill_rect.fill_rect(3.100000000000002, 5.299999999999998, 3.3000000000000016, 5.399999999999998);
        fill_rect.fill_rect(3.200000000000002, 5.299999999999998, 3.3000000000000016, 7.0);
        fill_rect.fill_rect(3.200000000000002, 6.9, 3.4000000000000017, 7.0);
        fill_rect.fill_rect(3.300000000000002, 5.350000000000001, 3.4000000000000017, 7.0);
        fill_rect.fill_rect(3.200000000000002, 1.0, 3.4000000000000017, 1.1);
        fill_rect.fill_rect(3.300000000000002, 1.0, 3.4000000000000017, 2.649999999999998);
        fill_rect.fill_rect(3.300000000000002, 1.0, 3.5000000000000018, 1.1);
        fill_rect.fill_rect(3.400000000000002, 1.0, 3.5000000000000018, 2.549999999999998);
        fill_rect.fill_rect(3.300000000000002, 5.499999999999997, 3.5000000000000018, 5.599999999999997);
        fill_rect.fill_rect(3.400000000000002, 5.499999999999997, 3.5000000000000018, 7.0);
        fill_rect.fill_rect(3.400000000000002, 6.9, 3.600000000000002, 7.0);
        fill_rect.fill_rect(3.500000000000002, 5.5500000000000025, 3.600000000000002, 7.0);
        fill_rect.fill_rect(3.400000000000002, 1.0, 3.600000000000002, 1.1);
        fill_rect.fill_rect(3.500000000000002, 1.0, 3.600000000000002, 2.449999999999998);
        fill_rect.fill_rect(3.500000000000002, 1.0, 3.700000000000002, 1.1);
        fill_rect.fill_rect(3.6000000000000023, 1.0, 3.700000000000002, 2.349999999999998);
        fill_rect.fill_rect(3.500000000000002, 5.699999999999997, 3.700000000000002, 5.799999999999996);
        fill_rect.fill_rect(3.6000000000000023, 5.699999999999997, 3.700000000000002, 7.0);
        fill_rect.fill_rect(3.6000000000000023, 6.9, 3.800000000000002, 7.0);
        fill_rect.fill_rect(3.7000000000000024, 5.750000000000002, 3.800000000000002, 7.0);
        fill_rect.fill_rect(3.6000000000000023, 1.0, 3.800000000000002, 1.1);
        fill_rect.fill_rect(3.7000000000000024, 1.0, 3.800000000000002, 2.249999999999998);
        fill_rect.fill_rect(3.7000000000000024, 1.0, 3.900000000000002, 1.1);
        fill_rect.fill_rect(3.8000000000000025, 1.0, 3.900000000000002, 2.1499999999999977);
        fill_rect.fill_rect(3.7000000000000024, 5.899999999999996, 3.900000000000002, 5.999999999999996);
        fill_rect.fill_rect(3.8000000000000025, 5.899999999999996, 3.900000000000002, 7.0);
        fill_rect.fill_rect(3.8000000000000025, 6.9, 4.000000000000003, 7.0);
        fill_rect.fill_rect(3.9000000000000026, 5.950000000000003, 4.000000000000003, 7.0);
        fill_rect.fill_rect(3.9000000000000026, 5.950000000000003, 4.100000000000002, 6.0500000000000025);
        fill_rect.fill_rect(4.000000000000003, 5.950000000000003, 4.100000000000002, 7.0);
        fill_rect.fill_rect(4.000000000000003, 6.9, 4.200000000000002, 7.0);
        fill_rect.fill_rect(4.100000000000002, 5.849999999999998, 4.200000000000002, 7.0);
        fill_rect.fill_rect(4.100000000000002, 5.849999999999998, 4.300000000000002, 5.9499999999999975);
        fill_rect.fill_rect(4.200000000000002, 5.849999999999998, 4.300000000000002, 7.0);
        fill_rect.fill_rect(4.200000000000002, 5.749999999999998, 4.300000000000002, 7.0);
        fill_rect.fill_rect(4.200000000000002, 5.749999999999998, 4.400000000000001, 5.849999999999998);
        fill_rect.fill_rect(4.300000000000002, 5.749999999999998, 4.400000000000001, 7.0);
        fill_rect.fill_rect(4.300000000000002, 5.649999999999999, 4.400000000000001, 7.0);
        fill_rect.fill_rect(4.300000000000002, 5.649999999999999, 4.500000000000001, 5.749999999999998);
        fill_rect.fill_rect(4.400000000000001, 5.649999999999999, 4.500000000000001, 7.0);
        fill_rect.fill_rect(4.400000000000001, 5.549999999999999, 4.500000000000001, 7.0);
        fill_rect.fill_rect(4.400000000000001, 5.549999999999999, 4.6000000000000005, 5.649999999999999);
        fill_rect.fill_rect(4.500000000000001, 5.549999999999999, 4.6000000000000005, 7.0);
        fill_rect.fill_rect(4.500000000000001, 5.449999999999999, 4.6000000000000005, 7.0);
        fill_rect.fill_rect(4.500000000000001, 5.449999999999999, 4.7, 5.549999999999999);
        fill_rect.fill_rect(4.6000000000000005, 5.449999999999999, 4.7, 7.0);
        fill_rect.fill_rect(4.6000000000000005, 5.35, 4.7, 7.0);
        fill_rect.fill_rect(4.6000000000000005, 5.35, 4.8, 5.449999999999999);
        fill_rect.fill_rect(4.7, 5.35, 4.8, 7.0);
        fill_rect.fill_rect(4.7, 5.25, 4.8, 7.0);
        fill_rect.fill_rect(4.7, 5.25, 4.8999999999999995, 5.35);
        fill_rect.fill_rect(4.8, 5.25, 4.8999999999999995, 7.0);
        fill_rect.fill_rect(4.8, 5.15, 4.8999999999999995, 7.0);
        fill_rect.fill_rect(4.8, 5.15, 4.999999999999999, 5.25);
        fill_rect.fill_rect(4.8999999999999995, 5.15, 4.999999999999999, 7.0);
        fill_rect.fill_rect(4.8999999999999995, 5.050000000000001, 4.999999999999999, 7.0);
        fill_rect.fill_rect(4.8999999999999995, 5.050000000000001, 5.099999999999999, 5.15);
        fill_rect.fill_rect(4.999999999999999, 5.050000000000001, 5.099999999999999, 7.0);
        fill_rect.fill_rect(4.999999999999999, 4.950000000000001, 5.099999999999999, 7.0);
        fill_rect.fill_rect(4.999999999999999, 4.950000000000001, 5.199999999999998, 5.050000000000001);
        fill_rect.fill_rect(5.099999999999999, 4.950000000000001, 5.199999999999998, 7.0);
        fill_rect.fill_rect(5.099999999999999, 4.850000000000001, 5.199999999999998, 7.0);
        fill_rect.fill_rect(5.099999999999999, 4.850000000000001, 5.299999999999998, 4.950000000000001);
        fill_rect.fill_rect(5.199999999999998, 4.850000000000001, 5.299999999999998, 7.0);
        fill_rect.fill_rect(5.199999999999998, 4.750000000000002, 5.299999999999998, 7.0);
        fill_rect.fill_rect(5.199999999999998, 4.750000000000002, 5.399999999999998, 4.850000000000001);
        fill_rect.fill_rect(5.299999999999998, 4.750000000000002, 5.399999999999998, 7.0);
        fill_rect.fill_rect(5.299999999999998, 4.650000000000002, 5.399999999999998, 7.0);
        fill_rect.fill_rect(5.299999999999998, 4.650000000000002, 5.499999999999997, 4.750000000000002);
        fill_rect.fill_rect(5.399999999999998, 4.650000000000002, 5.499999999999997, 7.0);
        fill_rect.fill_rect(5.399999999999998, 4.5500000000000025, 5.499999999999997, 7.0);
        fill_rect.fill_rect(5.399999999999998, 4.5500000000000025, 5.599999999999997, 4.650000000000002);
        fill_rect.fill_rect(5.499999999999997, 4.5500000000000025, 5.599999999999997, 7.0);
        fill_rect.fill_rect(5.499999999999997, 4.450000000000003, 5.599999999999997, 7.0);
        fill_rect.fill_rect(5.499999999999997, 4.450000000000003, 5.699999999999997, 4.5500000000000025);
        fill_rect.fill_rect(5.599999999999997, 4.450000000000003, 5.699999999999997, 7.0);
        fill_rect.fill_rect(5.599999999999997, 4.350000000000003, 5.699999999999997, 7.0);
        fill_rect.fill_rect(5.599999999999997, 4.350000000000003, 5.799999999999996, 4.450000000000003);
        fill_rect.fill_rect(5.699999999999997, 4.350000000000003, 5.799999999999996, 7.0);
        fill_rect.fill_rect(5.699999999999997, 4.2500000000000036, 5.799999999999996, 7.0);
        fill_rect.fill_rect(5.699999999999997, 4.2500000000000036, 5.899999999999996, 4.350000000000003);
        fill_rect.fill_rect(5.799999999999996, 4.2500000000000036, 5.899999999999996, 7.0);
        fill_rect.fill_rect(5.799999999999996, 4.150000000000004, 5.899999999999996, 7.0);
        fill_rect.fill_rect(5.799999999999996, 4.150000000000004, 5.999999999999996, 4.2500000000000036);
        fill_rect.fill_rect(5.899999999999996, 4.150000000000004, 5.999999999999996, 7.0);
        fill_rect.fill_rect(5.899999999999996, 4.050000000000004, 5.999999999999996, 7.0);
        fill_rect.fill_rect(5.899999999999996, 4.050000000000004, 6.099999999999995, 4.150000000000004);
        fill_rect.fill_rect(5.999999999999996, 4.050000000000004, 6.099999999999995, 7.0);
        fill_rect.fill_rect(5.999999999999996, 1.0, 6.099999999999995, 7.0);
        fill_rect.fill_rect(5.999999999999996, 1.0, 6.199999999999995, 1.1);
        fill_rect.fill_rect(6.099999999999995, 1.0, 6.199999999999995, 7.0);
        fill_rect.fill_rect(6.099999999999995, 6.9, 6.2999999999999945, 7.0);
        fill_rect.fill_rect(6.199999999999995, 1.0, 6.2999999999999945, 7.0);
        fill_rect.fill_rect(6.199999999999995, 1.0, 6.399999999999994, 1.1);
        fill_rect.fill_rect(6.2999999999999945, 1.0, 6.399999999999994, 7.0);
        fill_rect.fill_rect(6.2999999999999945, 6.9, 6.499999999999994, 7.0);
        fill_rect.fill_rect(6.399999999999994, 1.0, 6.499999999999994, 7.0);
        fill_rect.fill_rect(6.399999999999994, 1.0, 6.599999999999993, 1.1);
        fill_rect.fill_rect(6.499999999999994, 1.0, 6.599999999999993, 7.0);
        fill_rect.fill_rect(6.499999999999994, 6.9, 6.699999999999993, 7.0);
        fill_rect.fill_rect(6.599999999999993, 1.0, 6.699999999999993, 7.0);
        fill_rect.fill_rect(6.599999999999993, 1.0, 6.799999999999993, 1.1);
        fill_rect.fill_rect(6.699999999999993, 1.0, 6.799999999999993, 7.0);
        fill_rect.fill_rect(6.699999999999993, 6.9, 6.899999999999992, 7.0);
        fill_rect.fill_rect(6.799999999999993, 1.0, 6.899999999999992, 7.0);
        fill_rect.fill_rect(6.799999999999993, 1.0, 6.999999999999992, 1.1);
        fill_rect.fill_rect(6.899999999999992, 1.0, 6.999999999999992, 7.0);
        fill_rect.fill_rect(6.899999999999992, 6.9, 7.099999999999992, 7.0);
        fill_rect.fill_rect(6.999999999999992, 1.0, 7.099999999999992, 7.0);
        fill_rect.fill_rect(6.999999999999990, 0.9, 7.199999999999992, 7.1);
        assert_eq!(fill_rect.is_fill(6.999999999999992, 1.0, 7.0, 7.0), true);
        assert_eq!(fill_rect.is_fill(6.999999999999992, 1.0, 7.099999999999992, 7.0), true);
    }
}

