use std::cmp::Ordering;
use std::collections::HashMap;

pub struct ContinuousValueStats {
    mean: f64,
    median: f64,
    percentile_90th: f64,
}

impl Default for ContinuousValueStats {
    fn default() -> Self {
        Self {
            mean: 0f64,
            median: 0f64,
            percentile_90th: 0f64,
        }
    }
}

impl ContinuousValueStats {
    pub fn new(elements: &[f64]) -> Self {
        let mut sorted_elements = elements.to_owned();
        sorted_elements.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));

        Self {
            mean: sorted_elements.iter().sum::<f64>() / (sorted_elements.len() as f64),
            median: if sorted_elements.len() % 2 == 1 {
                sorted_elements[(sorted_elements.len() - 1) / 2]
            } else {
                (sorted_elements[sorted_elements.len() / 2]
                    + sorted_elements[sorted_elements.len() / 2 - 1])
                    / 2.0
            },
            percentile_90th: sorted_elements
                [(sorted_elements.len() as f64 * 0.9).floor() as usize],
        }
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn median(&self) -> f64 {
        self.median
    }

    pub fn percentile_90th(&self) -> f64 {
        self.percentile_90th
    }
}

pub struct CategoryStats<T: std::cmp::Eq + std::hash::Hash> {
    histogram: HashMap<T, i32>,
}

impl<T: std::cmp::Eq + std::hash::Hash> Default for CategoryStats<T> {
    fn default() -> Self {
        Self {
            histogram: HashMap::new(),
        }
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + std::fmt::Display> CategoryStats<T> {
    pub fn new(elements: Vec<T>) -> Self {
        let mut histogram = HashMap::new();
        for element in elements {
            histogram
                .entry(element)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        CategoryStats { histogram }
    }

    pub fn histogram_as_str(&self) -> String {
        self.histogram
            .iter()
            .map(|(element, count)| format!("[{}]: {}", element, count))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuous_value_stats_mean() {
        let elements = vec![10.0, 3.0, 1.0, 6.0];

        let stats = ContinuousValueStats::new(&elements);

        assert_eq!(5f64, stats.mean);
    }

    #[test]
    fn test_continuous_value_stats_median() {
        let elements = vec![3.0, 1.0, 6.0];

        let stats = ContinuousValueStats::new(&elements);

        assert_eq!(3.0, stats.median);
    }

    #[test]
    fn test_continuous_value_stats_median_for_even_number() {
        let elements = vec![10.0, 3.0, 1.0, 6.0];

        let stats = ContinuousValueStats::new(&elements);

        assert_eq!(4.5, stats.median);
    }

    #[test]
    fn test_continuous_value_stats_percentile_90th() {
        let elements = (0..5).map(|i| i as f64).rev().collect::<Vec<f64>>();

        let stats = ContinuousValueStats::new(&elements);

        assert_eq!(4.0, stats.percentile_90th);
    }

    #[test]
    fn test_continuous_value_stats_percentile_90th_boundary_value() {
        let elements = (0..100).map(|i| i as f64).collect::<Vec<f64>>();

        let stats = ContinuousValueStats::new(&elements);

        assert_eq!(90.0, stats.percentile_90th);
    }

    #[test]
    fn test_category_stats_histogram() {
        let elements = vec![2, 1, 2];

        let stats = CategoryStats::new(elements);

        assert_eq!(1, *stats.histogram.get(&1).unwrap());
        assert_eq!(2, *stats.histogram.get(&2).unwrap());
        assert_eq!(true, stats.histogram.get(&3).is_none())
    }
}
