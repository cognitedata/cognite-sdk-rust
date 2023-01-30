use std::collections::HashMap;
use std::hash::Hash;

pub fn chunk_map<'a, TKey: Hash + Eq + 'a + Clone, TValue: 'a>(
    inp: HashMap<TKey, Vec<TValue>>,
    max_total: usize,
    max_keys: usize,
) -> Box<dyn Iterator<Item = HashMap<TKey, Vec<TValue>>> + 'a> {
    assert!(max_keys > 0, "Max keys must be larger than 0");
    assert!(max_total > 0, "Max total must be larger than 0");
    let mut total = 0;
    let mut num_keys = 0;

    // First, check if we can just return the input. Since this case is common, it is worthwhile to check.
    for (_, value) in &inp {
        num_keys += 1;
        total += value.len();
    }
    if total < max_total && num_keys < max_keys {
        return Box::new(std::iter::once(inp));
    }

    // The default case, where we assume nothing, and do all checks.
    let mut res = vec![];
    let mut current = HashMap::new();
    let mut current_total = 0;
    for (key, mut values) in inp {
        if values.len() == 0 {
            continue;
        }
        if current.len() == max_keys {
            res.push(current);
            current = HashMap::new();
            current_total = 0;
        }

        // We overflowed the current list
        if current_total + values.len() > max_total {
            while values.len() > 0 {
                let to_add = (max_total - current_total).min(values.len());
                current.insert(key.clone(), values.drain(0..to_add).collect());
                current_total += to_add;
                if max_total - current_total == 0 {
                    res.push(current);
                    current = HashMap::new();
                    current_total = 0;
                }
            }
        } else {
            current_total += values.len();
            current.insert(key, values);
        }
    }
    if current.len() > 0 {
        res.push(current);
    }
    Box::new(res.into_iter())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::utils::chunk_map;

    fn chunk_map_t(
        data: HashMap<i64, Vec<i64>>,
        max_total: usize,
        max_keys: usize,
        expected_chunks: usize,
    ) {
        let total: usize = data.iter().map(|d| d.1.len()).sum();
        let res = chunk_map(data, max_total, max_keys).collect::<Vec<_>>();
        assert_eq!(expected_chunks, res.len());

        let new_total: usize = res
            .iter()
            .map(|c| c.iter().map(|d| d.1.len()).sum::<usize>())
            .sum();

        assert_eq!(total, new_total);

        for chunk in res {
            assert!(chunk.len() <= max_keys);
            let sum: usize = chunk.iter().map(|c| c.1.len()).sum();
            assert!(sum <= max_total);
        }
    }

    fn gen_chunk_test(num_keys: usize, data_per_key: usize) -> HashMap<i64, Vec<i64>> {
        let mut res = HashMap::new();
        for x in 0..num_keys {
            let data = vec![0i64; data_per_key];
            res.insert(x as i64, data);
        }
        res
    }

    #[test]
    pub fn test_chunk_map() {
        // No chunking should happen here
        chunk_map_t(gen_chunk_test(100, 100), 100_000, 10_000, 1);
        // Chunk on keys only
        chunk_map_t(gen_chunk_test(100_000, 1), 100_000, 10_000, 10);
        // Chunk because there is too much total data.
        chunk_map_t(gen_chunk_test(10_000, 100), 100_000, 10_000, 10);
        // Chunk on a few very large pieces
        chunk_map_t(gen_chunk_test(10, 1_000_000), 100_000, 10_000, 100);
        // Chunk on some irregular, large pieces
        chunk_map_t(gen_chunk_test(10, 888_888), 100_000, 10_000, 89);
    }
}
