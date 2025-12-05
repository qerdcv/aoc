fn merged_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.sort_unstable();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (s, e) in ranges {
        if let Some((_, last_e)) = merged.last_mut() {
            if s <= *last_e + 1 {
                *last_e = (*last_e).max(e);
            } else {
                merged.push((s, e));
            }
        } else {
            merged.push((s, e));
        }
    }

    merged
}

pub fn p1(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (start, stop) = line.split_once('-').unwrap();
        ranges.push((start.parse::<usize>().unwrap(), stop.parse::<usize>().unwrap()));
    }

    let merged = merged_ranges(ranges);
    lines
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<usize>().unwrap())
        .filter(|&i| merged.iter().any(|&(s, e)| i >= s && i <= e))
        .count() as i64
}

pub fn p2(input: &str) -> i64 {
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let (start, stop) = line.split_once('-').unwrap();
        let (start, stop) = (start.parse::<usize>().unwrap(), stop.parse::<usize>().unwrap());
        ranges.push((start, stop));
    }

    let merged = merged_ranges(ranges);
    merged
        .iter()
        .map(|&(start, stop)| (start..=stop).count())
        .sum::<usize>() as i64
}
