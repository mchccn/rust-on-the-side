#![allow(dead_code)]

#[derive(Debug)]
enum ChangeMode {
    Addition,
    Deletion,
    Edition,
}

#[derive(Debug)]
struct Change {
    mode: ChangeMode,
    index: i64,
    previous: String,
    current: String,
}

#[derive(Clone)]
struct IndexedPart(String, i64);

fn lcs(s1: &str, s2: &str) -> String {
    let mut arr: Vec<Vec<i64>> = Vec::new();
    
    for _ in 0..s2.len() + 1 { arr.push(vec![-1; s1.len() + 1]); }

    for i in 0..s1.len() + 1 { arr[0][i] = 0; }
    for j in 0..s2.len() + 1 { arr[j][0] = 0; }

    let mut len = 0;
    let mut col = 0;
    let mut row = 0;

    for i in 1..s2.len() + 1 {
        for j in 1..s1.len() + 1 {
            if s1.chars().nth(j - 1).unwrap() == s2.chars().nth(i - 1).unwrap() {
                arr[i][j] = arr[i - 1][j - 1] + 1;
            } else { arr[i][j] = 0; }

            if arr[i][j] > len {
                len = arr[i][j];
                col = j;
                row = i;
            }
        }
    }

    if len == 0 { return "".to_string() }

    let mut res = "".to_string();

    while arr[row][col] > 0 {
        res = format!("{}{}", s1.chars().nth(col - 1).unwrap(), res);
        row -= 1;
        col -= 1;
    }

    return res;
}

fn diff(source: &str, target: &str) -> Vec<Change> {
    let mut s1 = vec![IndexedPart(source.to_string(), 0)];
    let mut s2 = vec![IndexedPart(target.to_string(), 0)];

    'outer: loop {
        for i in 0..s1.len() {
            let IndexedPart(p, x) = &s1[i];

            if lcs(&p, &s2[i].0).len() > 1 {
                let mut d = vec![];

                for i in 0..s1.len() {
                    d.push(IndexedPart(lcs(&p, &s2[i].0), *x));
                }

                let mut l = 0;

                for i in 0..d.len() {
                    if d[i].0.len() > l { l = d[i].0.len(); }
                }

                let IndexedPart(s, x) = d.iter().find(|&x| x.0.len() == l).unwrap();

                if s.len() > 1 {
                    let mut s1p = vec![];
                    let mut s2p = vec![];

                    for i in 0..s1.len() {
                        let IndexedPart(c, v) = &s1[i];

                        if c.contains(s) {
                            s1p.push(IndexedPart(c.split(s).collect::<Vec<&str>>()[0].to_string(), *x));
                            s1p.push(IndexedPart(c.split(s).skip(1).collect::<Vec<&str>>().join(s), x + s.len() as i64 + c.split(s).collect::<Vec<&str>>()[0].len() as i64));
                        } else {
                            s1p.push(IndexedPart(c.to_string(), *v));
                        }
                    }

                    for i in 0..s2.len() {
                        let IndexedPart(c, v) = &s2[i];

                        if c.contains(s) {
                            s2p.push(IndexedPart(c.split(s).collect::<Vec<&str>>()[0].to_string(), *x));
                            s2p.push(IndexedPart(c.split(s).skip(1).collect::<Vec<&str>>().join(s), x + s.len() as i64 + c.split(s).collect::<Vec<&str>>()[0].len() as i64));
                        } else {
                            s2p.push(IndexedPart(c.to_string(), *v));
                        }
                    }

                    s1 = s1p;
                    s2 = s2p;
                }

                continue 'outer;
            }
        }

        break;
    }

    let mut changes = vec![];

    for i in 0..s1.len() {
        let a = s1[i].clone();
        let b = s2[i].clone();

        if a.0.len() != 0 || b.0.len() != 0 {
            let mut mode = ChangeMode::Edition;

            if a.0.len() > b.0.len() { mode = ChangeMode::Deletion; }

            if a.0.len() < b.0.len() { mode = ChangeMode::Addition; }

            changes.push(Change {
                mode,
                index: a.1,
                previous: a.0,
                current: b.0,
            });
        }
    }

    return changes;
}

fn main() {
    println!("{:?}", lcs("abc", "ab"));
    println!("{:?}", diff("abc", "ab"));
}
