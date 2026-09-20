//! Pure statistics over a set of real round-trip measurements. No
//! network access here — `main.rs` is the only part that opens a real
//! WebSocket connection.

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub min: Duration,
    pub avg: Duration,
    pub max: Duration,
    pub count: usize,
}

pub fn compute_stats(samples: &[Duration]) -> Option<Stats> {
    if samples.is_empty() {
        return None;
    }
    let min = *samples.iter().min().unwrap();
    let max = *samples.iter().max().unwrap();
    let total: Duration = samples.iter().sum();
    let avg = total / samples.len() as u32;
    Some(Stats {
        min,
        avg,
        max,
        count: samples.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_samples_yields_no_stats() {
        assert!(compute_stats(&[]).is_none());
    }

    #[test]
    fn single_sample_min_avg_and_max_are_all_equal() {
        let s = compute_stats(&[Duration::from_millis(10)]).unwrap();
        assert_eq!(s.min, Duration::from_millis(10));
        assert_eq!(s.avg, Duration::from_millis(10));
        assert_eq!(s.max, Duration::from_millis(10));
        assert_eq!(s.count, 1);
    }

    #[test]
    fn computes_correct_min_avg_max_over_multiple_samples() {
        let samples = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
        ];
        let s = compute_stats(&samples).unwrap();
        assert_eq!(s.min, Duration::from_millis(10));
        assert_eq!(s.avg, Duration::from_millis(20));
        assert_eq!(s.max, Duration::from_millis(30));
        assert_eq!(s.count, 3);
    }

    #[test]
    fn order_of_samples_does_not_affect_the_result() {
        let a = compute_stats(&[
            Duration::from_millis(5),
            Duration::from_millis(1),
            Duration::from_millis(3),
        ])
        .unwrap();
        let b = compute_stats(&[
            Duration::from_millis(1),
            Duration::from_millis(3),
            Duration::from_millis(5),
        ])
        .unwrap();
        assert_eq!(a, b);
    }
}
