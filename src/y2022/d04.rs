pub fn p1(input: &str) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut pairs = line.split(",");
        let mut a = pairs.next().unwrap().split('-');
        let mut b = pairs.next().unwrap().split('-');

        let a = (
            a
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            a
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        );
        let b = (
            b
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            b
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        );

        if a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1 {
            res += 1;
        }
    }

    res
}

pub fn p2(input: &str) -> i64 {
    let mut res = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut pairs = line.split(",");
        let mut a = pairs.next().unwrap().split('-');
        let mut b = pairs.next().unwrap().split('-');

        let a = (
            a
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            a
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        );
        let b = (
            b
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            b
                .next()
                .unwrap()
                .parse::<i64>()
                .unwrap()
        );

        if a.0 >= b.0 && a.0 <= b.1 || b.0 >= a.0 && b.0 <= a.1 {
            res += 1;
        }
    }

    res
}
