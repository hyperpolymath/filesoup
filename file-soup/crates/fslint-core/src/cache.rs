use fslint_plugin_api::PluginResult;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Cache key: (path, modified_time, size)
pub type CacheKey = (PathBuf, Option<SystemTime>, u64);

/// Result cache for plugin execution
pub struct ResultCache {
    cache: HashMap<CacheKey, Vec<PluginResult>>,
    hits: usize,
    misses: usize,
}

impl ResultCache {
    /// Create a new result cache
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Get cached results
    pub fn get(&mut self, key: &CacheKey) -> Option<Vec<PluginResult>> {
        match self.cache.get(key) {
            Some(results) => {
                self.hits += 1;
                Some(results.clone())
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    /// Insert results into cache
    pub fn insert(&mut self, key: CacheKey, results: Vec<PluginResult>) {
        self.cache.insert(key, results);
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Get cache statistics (hits, misses)
    pub fn stats(&self) -> (usize, usize) {
        (self.hits, self.misses)
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Get hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

impl Default for ResultCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fslint_plugin_api::{PluginResult, PluginStatus};

    #[test]
    fn test_cache_basic() {
        let mut cache = ResultCache::new();
        let key = (PathBuf::from("/test"), None, 0);

        let result = PluginResult {
            plugin_name: "test".into(),
            status: PluginStatus::Active,
            message: None,
            color: None,
            tags: vec![],
            metadata: HashMap::new(),
        };

        assert!(cache.get(&key).is_none());
        cache.insert(key.clone(), vec![result.clone()]);
        assert!(cache.get(&key).is_some());

        let (hits, misses) = cache.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }

    #[test]
    fn test_cache_hit_rate() {
        let mut cache = ResultCache::new();
        let key = (PathBuf::from("/test"), None, 0);

        let result = PluginResult {
            plugin_name: "test".into(),
            status: PluginStatus::Active,
            message: None,
            color: None,
            tags: vec![],
            metadata: HashMap::new(),
        };

        cache.insert(key.clone(), vec![result]);
        cache.get(&key); // hit
        cache.get(&key); // hit
        cache.get(&(PathBuf::from("/other"), None, 0)); // miss

        assert_eq!(cache.hit_rate(), 2.0 / 3.0);
    }
}
