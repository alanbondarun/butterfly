use std::collections::HashMap;

pub struct ContinuousValueStats {
    mean: f64,
}

impl Default for ContinuousValueStats {
    fn default() -> Self {
        Self { mean: 0f64 }
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

impl<T: std::cmp::Eq + std::hash::Hash> CategoryStats<T> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_stats_new() {
        let elements = vec![2, 1, 2];

        let stats = CategoryStats::new(elements);

        assert_eq!(1, *stats.histogram.get(&1).unwrap());
        assert_eq!(2, *stats.histogram.get(&2).unwrap());
        assert_eq!(true, stats.histogram.get(&3).is_none())
    }
}
