use std::{fs, vec};

enum InputType {
    Dummy,
    Real,
}

fn first_part(input: InputType) {
    let range_path = match input {
        InputType::Dummy => "dummy_input.txt",
        InputType::Real => "input.txt",
    };

    let id_path = match input {
        InputType::Dummy => "dummy_ids.txt",
        InputType::Real => "ids.txt",
    };

    let range_content = fs::read_to_string(range_path).expect("Test");
    let id_content = fs::read_to_string(id_path).expect("test");

    let mut ranges: Vec<Vec<i64>> = vec![];

    for line in range_content.lines() {
        let range_values: Vec<i64> = line.split('-').map(|n| n.parse::<i64>().unwrap()).collect();
        ranges.push(vec![range_values[0], range_values[1]]);
    }

    // let mut fresh_ids = HashSet::new();

    // for line in range_content.lines() {
    //     let ranges: Vec<i64> = line.split('-').map(|n| n.parse::<i64>().unwrap()).collect();
    //     fresh_ids.extend(ranges[0]..(ranges[1] + 1));
    // }

    let mut sum = 0;

    for line in id_content.lines() {
        let id: i64 = line.parse().unwrap();

        for range in ranges.iter() {
            if range[0] <= id && id <= range[1] {
                sum += 1;
                break;
            }
        }
    }

    println!("{}", sum)
}

#[derive(Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn intersection(&self, range: &Range) -> std::ops::Range<i64> {
        let start = self.start.max(range.start);
        let end = self.end.min(range.end);

        start..end
    }

    fn contains(&self, range: &Range) -> bool {
        return self.start <= range.start && self.end >= range.end;
    }

    // fn remove_intersection(self, range: &Range) -> Vec<Range> {
    //     if self.intersection(range).is_empty() {
    //         return vec![self.clone()];
    //     }
    //
    //     if self.contains(range) {
    //         if self.start == range.start && self.end == range.end {
    //             return vec![];
    //         } else {
    //             let mut remaining = vec![];
    //
    //             if self.start <= range.start {
    //                 remaining.push(Range {
    //                     start: self.start,
    //                     end: range.start - 1,
    //                 })
    //             }
    //
    //             if self.end >= range.end {
    //                 remaining.push(Range {
    //                     start: range.end + 1,
    //                     end: self.end,
    //                 })
    //             }
    //             return remaining;
    //         }
    //     }
    //
    //     let mut new_start = self.start;
    //     let mut new_end = self.end;
    //
    //     match range.end {
    //         end if end >= self.start => new_start = end.min(self.end),
    //         _ => new_end = range.start.max(self.start),
    //     }
    //
    //     if self.start < range.end {
    //         new_start = range.end
    //     }
    //
    //     return vec![Range {
    //         start: new_start,
    //         end: new_end,
    //     }];
    // }

    fn remove_intersection(&self, other: &Range) -> Vec<Range> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        // No intersection
        if start > end {
            return vec![self.clone()];
        }

        let mut result = vec![];

        // Left part
        if self.start < start {
            result.push(Range {
                start: self.start,
                end: start - 1,
            });
        }

        // Right part
        if self.end > end {
            result.push(Range {
                start: end + 1,
                end: self.end,
            });
        }

        result
    }

    fn find_sum(&self) -> i64 {
        if self.end == self.start {
            return 0;
        }

        self.end - self.start + 1
    }
}

fn second_part(input: InputType) {
    let path = match input {
        InputType::Dummy => "dummy_input.txt",
        InputType::Real => "input.txt",
    };

    let content = fs::read_to_string(path).expect("Test");

    let mut ranges: Vec<Range> = vec![];

    for line in content.lines() {
        let range_values: Vec<i64> = line.split('-').map(|n| n.parse::<i64>().unwrap()).collect();
        ranges.push(Range {
            start: range_values[0],
            end: range_values[1],
        });
    }

    let mut final_ranges = vec![];

    let mut sum = 0;

    while let Some(current) = ranges.pop() {
        let mut temp_ranges = vec![current];

        for existing in &final_ranges {
            let mut new_temp = Vec::new();
            for r in temp_ranges {
                // split r by existing range
                new_temp.extend(r.remove_intersection(existing));
            }
            temp_ranges = new_temp;
        }

        // add remaining parts to final_ranges
        final_ranges.extend(temp_ranges);
    }

    for range in final_ranges {
        sum += range.find_sum();
    }

    // for i in 0..ranges.len() {
    //     let (left, right) = ranges.split_at_mut(i);
    //
    //     let current = &mut right[0];
    //
    //     for range in left {
    //         current.remove_intersection(range);
    //     }
    //
    //     sum += current.find_sum();
    // }
    // for (i, range) in ranges.iter_mut().enumerate() {
    //     for j in 0..i {
    //         range.remove_intersection(&ranges[j]);
    //     }
    //
    //     sum += range.find_sum()
    // }

    println!("{}", sum)
}

fn main() {
    // first_part(InputType::Dummy);
    // first_part(InputType::Dummy);
    // second_part(InputType::Dummy);

    second_part(InputType::Real);
    // second_part(InputType::Real);
}
