use fslint_core::ScannedFile;

#[derive(Debug, Clone)]
pub struct Query {
    filters: Vec<Filter>,
}

#[derive(Debug, Clone)]
enum Filter {
    Name(String),
    Extension(String),
    Newest(bool),
    Plugin(String, String), // plugin_name, expected_value
    Tag(String),
    SizeLt(u64),
    SizeGt(u64),
}

impl Query {
    pub fn parse(query_str: &str) -> Result<Self, String> {
        let mut filters = Vec::new();

        for part in query_str.split_whitespace() {
            if let Some((key, value)) = part.split_once(':') {
                let filter = match key {
                    "name" => Filter::Name(value.to_string()),
                    "ext" => Filter::Extension(value.to_string()),
                    "newest" => Filter::Newest(value == "true"),
                    "tag" => Filter::Tag(value.to_string()),
                    "size_lt" => Filter::SizeLt(value.parse().map_err(|_| "Invalid size")?),
                    "size_gt" => Filter::SizeGt(value.parse().map_err(|_| "Invalid size")?),
                    plugin_name => {
                        // Treat unknown keys as plugin filters
                        Filter::Plugin(plugin_name.to_string(), value.to_string())
                    }
                };
                filters.push(filter);
            } else {
                return Err(format!("Invalid query part: {}", part));
            }
        }

        Ok(Self { filters })
    }

    pub fn apply(&self, files: Vec<ScannedFile>) -> Vec<ScannedFile> {
        let mut result: Vec<ScannedFile> = files
            .into_iter()
            .filter(|file| self.matches(file))
            .collect();

        // Handle "newest" filter
        if self.filters.iter().any(|f| matches!(f, Filter::Newest(true))) {
            result.sort_by(|a, b| {
                b.metadata.modified().unwrap()
                    .cmp(&a.metadata.modified().unwrap())
            });
            if !result.is_empty() {
                result = vec![result[0].clone()];
            }
        }

        result
    }

    fn matches(&self, file: &ScannedFile) -> bool {
        for filter in &self.filters {
            match filter {
                Filter::Name(name) => {
                    let filename = file.path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    if !filename.contains(name) {
                        return false;
                    }
                }
                Filter::Extension(ext) => {
                    let file_ext = file.path.extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("");
                    if file_ext != ext {
                        return false;
                    }
                }
                Filter::Tag(tag) => {
                    let has_tag = file.results.iter()
                        .any(|r| r.tags.contains(tag));
                    if !has_tag {
                        return false;
                    }
                }
                Filter::Plugin(plugin_name, expected_value) => {
                    let matches = file.results.iter()
                        .find(|r| &r.plugin_name == plugin_name)
                        .and_then(|r| r.message.as_ref())
                        .map(|msg| msg.contains(expected_value))
                        .unwrap_or(false);
                    if !matches {
                        return false;
                    }
                }
                Filter::SizeLt(size) => {
                    if file.metadata.len() >= *size {
                        return false;
                    }
                }
                Filter::SizeGt(size) => {
                    if file.metadata.len() <= *size {
                        return false;
                    }
                }
                Filter::Newest(_) => {
                    // Handled separately in apply()
                    continue;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_parse() {
        let query = Query::parse("name:test ext:txt").unwrap();
        assert_eq!(query.filters.len(), 2);
    }

    #[test]
    fn test_query_parse_plugin_filter() {
        let query = Query::parse("git-status:Modified").unwrap();
        assert_eq!(query.filters.len(), 1);
    }

    #[test]
    fn test_query_parse_size() {
        let query = Query::parse("size_gt:1024 size_lt:2048").unwrap();
        assert_eq!(query.filters.len(), 2);
    }

    #[test]
    fn test_query_parse_error() {
        assert!(Query::parse("invalid_query").is_err());
    }
}
