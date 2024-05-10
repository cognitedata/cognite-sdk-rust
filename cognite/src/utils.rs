use std::collections::HashMap;
use std::hash::Hash;

use futures::stream::FuturesUnordered;
use futures::{Future, StreamExt};

/// Chunk a map of lists both by the maximum number of keys per map,
/// and the number of total values.
///
/// # Arguments
///
/// * `inp` - The input hashmap.
/// * `max_total` - Maximum number of total values per result map.
/// * `max_keys` - Maximum number of keys per result map.
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
    for value in inp.values() {
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
        if values.is_empty() {
            continue;
        }
        if current.len() == max_keys {
            res.push(current);
            current = HashMap::new();
            current_total = 0;
        }

        // We overflowed the current list
        if current_total + values.len() > max_total {
            while !values.is_empty() {
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
    if !current.is_empty() {
        res.push(current);
    }
    Box::new(res.into_iter())
}

/// Execute a list of futures concurrently, with a maximum parallelism.
///
/// # Arguments
///
/// * `futures` - Iterator over futures to execute.
/// * `parallelism` - Number of futures to execute in parallel. Must be greater than zero.
///
/// # Panics
///
/// This function panics if parallelism is 0.
pub async fn execute_with_parallelism<T, TErr>(
    mut futures: impl Iterator<Item = impl Future<Output = Result<T, TErr>>>,
    parallelism: usize,
) -> Result<Vec<T>, TErr> {
    let mut res = Vec::new();

    assert!(parallelism > 0, "Parallelism must be greater than 0");

    let mut running = FuturesUnordered::new();

    for fut in (&mut futures).take(parallelism) {
        running.push(fut);
    }

    while let Some(r) = running.next().await {
        res.push(r?);

        if let Some(fut) = futures.next() {
            running.push(fut);
        }
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use std::sync::atomic::Ordering;
    use std::{collections::HashMap, sync::atomic::AtomicU64, time::Duration};

    use crate::utils::chunk_map;

    use crate::utils::execute_with_parallelism;

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

    #[tokio::test]
    pub async fn test_run_parallel() {
        let active = AtomicU64::new(0);
        let count = AtomicU64::new(0);
        let gen = || async {
            let initial = active.fetch_add(1, std::sync::atomic::Ordering::Acquire);
            assert!(initial < 4);
            tokio::time::sleep(Duration::from_millis(200)).await;
            let fin = active.fetch_sub(1, std::sync::atomic::Ordering::Release);
            assert!(fin <= 4 && fin > 0);

            count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Result::<(), ()>::Ok(())
        };

        let futures = (0..10).map(|_| gen());
        execute_with_parallelism(futures, 4).await.unwrap();

        let c = count.load(Ordering::Relaxed);
        assert_eq!(c, 10);
    }

    #[tokio::test]
    pub async fn test_run_parallel_small() {
        let active = AtomicU64::new(0);
        let count = AtomicU64::new(0);
        let gen = || async {
            let initial = active.fetch_add(1, std::sync::atomic::Ordering::Acquire);
            assert!(initial < 4);
            tokio::time::sleep(Duration::from_millis(200)).await;
            let fin = active.fetch_sub(1, std::sync::atomic::Ordering::Release);
            assert!(fin <= 4 && fin > 0);

            count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Result::<(), ()>::Ok(())
        };

        let futures = (0..3).map(|_| gen());
        execute_with_parallelism(futures, 4).await.unwrap();

        let c = count.load(Ordering::Relaxed);
        assert_eq!(c, 3);
    }

    #[tokio::test]
    pub async fn test_run_parallel_early_fail() {
        let active = AtomicU64::new(0);
        let count = AtomicU64::new(0);
        let gen = || async {
            let initial = active.fetch_add(1, std::sync::atomic::Ordering::Acquire);
            assert!(initial < 4);
            tokio::time::sleep(Duration::from_millis(200)).await;
            let fin = active.fetch_sub(1, std::sync::atomic::Ordering::Release);
            assert!(fin <= 4 && fin > 0);

            let c = count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

            if c == 5 {
                Result::<(), ()>::Err(())
            } else {
                Result::<(), ()>::Ok(())
            }
        };

        let futures = (0..10).map(|_| gen());
        assert!(execute_with_parallelism(futures, 4).await.is_err());

        // This should actually work consistently, the final part of the last future shouldn't be ran
        // after the first future fails. If this fails flakily, change this to a >= instead,
        // though my understanding of Futures in rust is that it shouldn't.
        let c = count.load(Ordering::Relaxed);
        assert_eq!(c, 6);
    }
}
