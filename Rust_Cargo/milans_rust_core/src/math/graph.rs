pub fn floyd_warshall<T>(adjecency_matrix: Vec<Vec<Option<T>>>) -> Vec<Vec<Option<(T, usize)>>>
where
    T: Clone + std::ops::Add<T, Output = T> + PartialOrd + std::fmt::Debug,
{
    if adjecency_matrix.is_empty() {
        return vec![];
    }
    let length = match adjecency_matrix.iter().map(|x| x.len()).max() {
        None => {
            std::unreachable!()
        }
        Some(length) => length.max(adjecency_matrix.len()),
    };
    let mut result = vec![vec![None; length]; length];
    for (start, array) in adjecency_matrix.iter().enumerate() {
        for (dest, distance) in array.iter().enumerate() {
            match distance.clone() {
                None => {}
                Some(distance) => result[start][dest] = Some((distance, start)),
            }
        }
    }
    for via in 0..length {
        for start in 0..length {
            if start == via {
                continue;
            }
            for dest in 0..length {
                if dest == via {
                    continue;
                }
                let (via1, via2) = if let (Some(via1), Some(via2)) =
                    (result[start][via].clone(), result[via][dest].clone())
                {
                    (via1, via2)
                } else {
                    continue;
                };
                if let Some(distance) = result[start][dest].clone() {
                    if via1.0.clone() + via2.0.clone() < distance.0 {
                        result[start][dest] = Some((via1.0 + via2.0, via))
                    }
                } else {
                    result[start][dest] = Some((via1.0 + via2.0, via2.1))
                }
            }
        }
    }
    for val in 0..length {
        result[val][val] = None;
    }
    result
}
