use std::collections::{BTreeMap, HashMap};

/// Per time series: BTreeMap<timestamp_ms, f64> for O(log n + k) range queries.
/// Insert is an upsert — existing timestamps are overwritten.
#[derive(Default)]
pub struct DatapointStore {
    /// ts_id → timestamp_ms → value
    pub series: HashMap<i64, BTreeMap<i64, f64>>,
}

impl DatapointStore {
    pub fn upsert(&mut self, ts_id: i64, points: impl IntoIterator<Item = (i64, f64)>) {
        let map = self.series.entry(ts_id).or_default();
        for (ts, val) in points {
            map.insert(ts, val);
        }
    }

    /// Raw points in [start, end). `limit` caps the result.
    pub fn retrieve_raw(
        &self,
        ts_id: i64,
        start: Option<i64>,
        end: Option<i64>,
        limit: usize,
    ) -> Vec<(i64, f64)> {
        let Some(map) = self.series.get(&ts_id) else {
            return vec![];
        };
        let iter = match (start, end) {
            (Some(s), Some(e)) => map.range(s..e),
            (Some(s), None) => map.range(s..),
            (None, Some(e)) => map.range(..e),
            (None, None) => map.range(..),
        };
        iter.take(limit).map(|(&ts, &v)| (ts, v)).collect()
    }

    /// Latest point strictly before `before_ms` (or the global latest).
    pub fn retrieve_latest(&self, ts_id: i64, before: Option<i64>) -> Option<(i64, f64)> {
        let map = self.series.get(&ts_id)?;
        let entry = match before {
            Some(b) => map.range(..b).next_back(),
            None => map.iter().next_back(),
        };
        entry.map(|(&ts, &v)| (ts, v))
    }

    /// Aggregate computation over [start, end) in granularity_ms buckets.
    /// Supported aggregates: "average", "min", "max", "count", "sum",
    /// "interpolation", "stepInterpolation".
    pub fn retrieve_aggregates(
        &self,
        ts_id: i64,
        start: i64,
        end: i64,
        granularity_ms: i64,
        aggregates: &[String],
    ) -> Vec<AggregatePoint> {
        let Some(map) = self.series.get(&ts_id) else {
            return vec![];
        };
        if granularity_ms <= 0 {
            return vec![];
        }

        let want = |name: &str| aggregates.iter().any(|a| a == name);
        let mut results = Vec::new();
        let mut bucket_start = start;

        while bucket_start < end {
            let bucket_end = (bucket_start + granularity_ms).min(end);
            let bucket_points: Vec<(i64, f64)> = map
                .range(bucket_start..bucket_end)
                .map(|(&ts, &v)| (ts, v))
                .collect();
            let count = bucket_points.len() as i64;
            let interp_value = if want("interpolation") || want("stepInterpolation") {
                compute_interpolation(map, bucket_start)
            } else {
                None
            };
            let (sum, avg, min_val, max_val) = if !bucket_points.is_empty() {
                let sum: f64 = bucket_points.iter().map(|(_, v)| v).sum();
                let min_val = bucket_points
                    .iter()
                    .map(|(_, v)| *v)
                    .fold(f64::INFINITY, f64::min);
                let max_val = bucket_points
                    .iter()
                    .map(|(_, v)| *v)
                    .fold(f64::NEG_INFINITY, f64::max);
                (sum, sum / count as f64, min_val, max_val)
            } else {
                (0.0, 0.0, 0.0, 0.0)
            };

            let mut point = AggregatePoint {
                timestamp: bucket_start,
                average: None,
                min: None,
                max: None,
                count: None,
                sum: None,
                interpolation: None,
                step_interpolation: None,
            };
            if want("average") {
                point.average = Some(avg);
            }
            if want("min") {
                point.min = Some(min_val);
            }
            if want("max") {
                point.max = Some(max_val);
            }
            if want("count") {
                point.count = Some(count);
            }
            if want("sum") {
                point.sum = Some(sum);
            }
            if want("interpolation") {
                point.interpolation = interp_value;
            }
            if want("stepInterpolation") {
                point.step_interpolation = interp_value;
            }
            results.push(point);
            bucket_start += granularity_ms;
        }
        results
    }
}

fn compute_interpolation(map: &BTreeMap<i64, f64>, t: i64) -> Option<f64> {
    let before = map.range(..=t).next_back()?;
    let (&t0, &v0) = before;
    if t0 == t {
        return Some(v0);
    }
    match map.range(t..).next() {
        Some((&t1, &v1)) if t1 > t0 => {
            let frac = (t - t0) as f64 / (t1 - t0) as f64;
            Some(v0 + frac * (v1 - v0))
        }
        _ => Some(v0),
    }
}

/// Parse a CDF granularity string into milliseconds.
/// Supported: "Xs", "Xm", "Xh", "Xd".
pub fn parse_granularity(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let unit = s.chars().last()?;
    let n: i64 = s[..s.len() - 1].parse().ok()?;
    if n <= 0 {
        return None;
    }
    match unit {
        's' => Some(n * 1_000),
        'm' => Some(n * 60_000),
        'h' => Some(n * 3_600_000),
        'd' => Some(n * 86_400_000),
        _ => None,
    }
}

pub struct AggregatePoint {
    pub timestamp: i64,
    pub average: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub count: Option<i64>,
    pub sum: Option<f64>,
    pub interpolation: Option<f64>,
    pub step_interpolation: Option<f64>,
}
