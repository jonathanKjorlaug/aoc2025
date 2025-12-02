use std::fs;

struct RangePair {
    first: i64,
    last: i64,
}

impl RangePair {
    fn new(first: i64, last: i64) -> RangePair {
        assert!(first < last, "End value must be larger than starting value");
        Self { first, last }
    }

    fn to_range(&self) -> std::ops::Range<i64> {
        self.first..(self.last + 1)
    }
}

fn all_equal(chunks: Vec<String>) -> bool {
    let mut sum = 0;

    for element in chunks.iter() {
        if *element == chunks[0] {
            sum += 1;
        }
    }

    return sum == chunks.len();
}

fn is_fake(id: i64) -> bool {
    let idstring = id.to_string();
    let length = idstring.len();

    for seqence_length in 0..(length / 2) {
        let chunks: Vec<_> = idstring
            .chars()
            .collect::<Vec<_>>()
            .chunks(seqence_length + 1)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        if all_equal(chunks) {
            return true;
        }
    }

    return false;
}

fn main() {
    let file_info = fs::read_to_string("input.txt").expect("Test");
    let contents = file_info.trim_end();

    let ranges: Vec<RangePair> = contents
        .split(',')
        .map(|group| {
            let nums: Vec<i64> = group
                .split('-')
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            RangePair::new(nums[0], nums[1]) // invariant is enforced here
        })
        .collect();
    let mut ids: Vec<i64> = vec![];

    for r in ranges.iter() {
        ids.extend(r.to_range());
    }

    let filtered_ids: Vec<&i64> = ids.iter().filter(|id| is_fake(**id)).collect();

    let sum: i64 = filtered_ids.iter().copied().sum();
    println!("{sum}");
}
