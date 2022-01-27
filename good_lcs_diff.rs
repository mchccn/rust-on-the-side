#![allow(dead_code)]

/**
 * Authored by ThatsNoMoon @ https://github.com/ThatsNoMoon
 */

//

#[derive(Debug)]
enum ChangeMode {
    Addition,
    Deletion,
    Edition,
}

#[derive(Debug)]
struct Change {
    mode: ChangeMode,
    index: usize,
    previous: String,
    current: String,
}

#[derive(Clone)]
struct IndexedPart(String, usize);

fn lcs(s1: &str, s2: &str) -> String {
    let mut arr = vec![];

    for _ in 0..s2.len() + 1 {
        arr.push(vec![-1i64; s1.len() + 1]);
    }

    for i in 0..s1.len() + 1 {
        arr[0][i] = 0;
    }
    for j in 0..s2.len() + 1 {
        arr[j][0] = 0;
    }

    let mut len = 0;
    let mut col = 0;
    let mut row = 0;

    for (i, c2) in s2.chars().enumerate().map(|(i, c)| (i + 1, c)) {
        for (j, c1) in s1.chars().enumerate().map(|(j, c)| (j + 1, c)) {
            arr[i][j] = if c1 == c2 { arr[i - 1][j - 1] + 1 } else { 0 };

            if arr[i][j] > len {
                len = arr[i][j];
                col = j;
                row = i;
            }
        }
    }

    if len == 0 {
        return "".to_string();
    }

    let mut res = vec![];
    let mut chars = s1.chars().rev().skip(s1.len() - col);

    while arr[row][col] > 0 {
        res.push(chars.next().unwrap());
        row -= 1;
        col -= 1;
    }

    return res.into_iter().rev().collect();
}

fn diff(source: &str, target: &str) -> Vec<Change> {
    let mut s1 = vec![IndexedPart(source.into(), 0)];
    let mut s2 = vec![IndexedPart(target.into(), 0)];

    while s1
        .iter()
        .enumerate()
        .any(|(i, IndexedPart(p, _))| lcs(&p, &s2[i].0).len() > 1)
    {
        let IndexedPart(s, x) = s1
            .iter()
            .enumerate()
            .map(|(i, IndexedPart(p, x))| IndexedPart(lcs(&p, &s2[i].0), *x))
            .max_by_key(|x| x.0.len())
            .unwrap();

        if s.len() > 1 {
            let f =
                |IndexedPart(c, v)| match c.split_once(&s) {
                    Some((prefix, rest)) => Some(IndexedPart(prefix.to_owned(), x))
                        .into_iter()
                        .chain(Some(IndexedPart(
                            rest.to_owned(),
                            x + s.len() + prefix.len(),
                        ))),
                    None => Some(IndexedPart(c, v)).into_iter().chain(None),
                };

            s1 = s1.into_iter().flat_map(f).collect();

            s2 = s2.into_iter().flat_map(f).collect();
        }
    }

    s1.into_iter()
        .zip(s2)
        .filter(|(a, b)| a.0.len() != 0 || b.0.len() != 0)
        .map(|(a, b)| {
            let mode = if a.0.len() > b.0.len() {
                ChangeMode::Deletion
            } else if a.0.len() < b.0.len() {
                ChangeMode::Addition
            } else {
                ChangeMode::Edition
            };

            Change {
                mode,
                index: a.1,
                previous: a.0,
                current: b.0,
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", lcs("abc", "ab"));
    println!("{:?}", diff("abc", "ab"));
}
