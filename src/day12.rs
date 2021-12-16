use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[allow(dead_code)]
pub fn part1<R: BufRead>(reader: R) -> u32 {
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for res in reader.lines() {
        let line = res.unwrap();
        let (a, b) = line.split_once('-').unwrap();
        edges.entry(a.to_string()).or_default().push(b.to_string());
        edges.entry(b.to_string()).or_default().push(a.to_string());
    }

    let mut path = Vec::new();
    path.push((String::from("start"), 0));

    let mut count = 0;
    loop {
        let (last, last_idx) = match path.pop() {
            None => break,
            Some(v) => v,
        };

        let next = match edges.get(&last).map(|l| l.get(last_idx)).flatten() {
            None => continue,
            Some(next) => next,
        };

        path.push((last, last_idx + 1));

        if next.as_str() == "end" {
            count += 1;
            continue;
        }

        if !next.chars().all(|c| c.is_ascii_uppercase()) && path.iter().any(|(v, _)| v == next) {
            continue;
        }

        path.push((next.clone(), 0));
    }

    count
}

#[allow(dead_code)]
pub fn part2<R: BufRead>(reader: R) -> u32 {
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for res in reader.lines() {
        let line = res.unwrap();
        let (a, b) = line.split_once('-').unwrap();
        edges.entry(a.to_string()).or_default().push(b.to_string());
        edges.entry(b.to_string()).or_default().push(a.to_string());
    }

    let mut path = Vec::new();
    path.push((String::from("start"), 0));

    let mut count = 0;
    loop {
        let (last, last_idx) = match path.pop() {
            None => break,
            Some(v) => v,
        };

        let next = match edges.get(&last).map(|l| l.get(last_idx)).flatten() {
            None => continue,
            Some(next) => next,
        };

        path.push((last, last_idx + 1));

        if next.as_str() == "end" {
            count += 1;
            continue;
        }

        if next.as_str() == "start" {
            continue;
        }

        let next_is_uppercase = || next.chars().all(|c| c.is_ascii_uppercase());
        let next_in_path = || path.iter().any(|(v, _)| v == next);
        let path_has_dupe = || {
            let mut existing = HashSet::new();
            for (seg, _) in path.iter() {
                if seg.chars().all(|c| c.is_ascii_uppercase()) {
                    continue;
                }
                if existing.contains(seg) {
                    return true;
                }
                existing.insert(seg);
            }
            false
        };

        if next_is_uppercase() || !next_in_path() || !path_has_dupe() {
            path.push((next.clone(), 0));
        }
    }

    count
}
