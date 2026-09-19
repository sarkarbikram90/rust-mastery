// src/main.rs
//! High-Performance Time-Series Database for AI Observability (Mock).

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

// --- 1. Core Data Structures ---

#[derive(Debug, Clone, Serialize)]
pub struct MetricPoint {
    pub timestamp: u64, // Unix Nano
    pub value: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimeSeries {
    pub name: String,
    pub tags: Vec<Tag>,
    // BTreeMap keeps keys (timestamps) sorted automatically for fast range scans.
    pub points: BTreeMap<u64, f64>,
}

impl TimeSeries {
    pub fn new(name: &str, tags: Vec<Tag>) -> Self {
        Self {
            name: name.to_string(),
            tags,
            points: BTreeMap::new(),
        }
    }

    pub fn add_point(&mut self, timestamp: u64, value: f64) {
        self.points.insert(timestamp, value);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DataPoint {
    pub resource_id: String, // e.g., GPU_ID_0
    pub metric_name: String,
    pub timestamp_unix_nano: u64,
    pub value: f64,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Query {
    pub metric_name: String,
    pub start: u64,
    pub end: u64,
    #[serde(default)]
    pub group_by_tags: Vec<String>,
}

// --- 2. The Database Engine (In-Memory) ---

pub struct TSDB {
    // We use Arc<Mutex<...>> to allow safe concurrent access from multiple threads (e.g., API + Metrics ingestion)
    // The outer HashMap keys are Metric Names.
    // The inner BTreeMap keys are Resource IDs.
    data: Arc<Mutex<HashMap<String, BTreeMap<String, TimeSeries>>>>,
}

impl TSDB {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Fast path: Insert a raw data point directly.
    pub fn ingest(&self, point: DataPoint) {
        let mut data = self.data.lock().unwrap();

        // Get the map for this metric name (e.g., "gpu.duty_cycle")
        // Or insert a new BTreeMap if it doesn't exist.
        let metric_map = data.entry(point.metric_name.clone()).or_default();

        // Get the specific series for this resource (e.g., "GPU_ID_0")
        // Or insert a new TimeSeries if it doesn't exist.
        let series = metric_map
            .entry(point.resource_id)
            .or_insert_with(|| TimeSeries::new(&point.metric_name, point.tags.clone()));

        // Insert the actual value
        series.add_point(point.timestamp_unix_nano, point.value);
    }

    /// Query/Rollup Logic
    pub fn query(&self, query: &Query) -> Result<Vec<TimeSeries>, String> {
        let data = self.data.lock().unwrap();

        // 1. Find the metric map
        let metric_map = data
            .get(&query.metric_name)
            .ok_or_else(|| format!("Metric {} not found", query.metric_name))?;

        let mut results = HashMap::new(); // Temporary map for grouping

        // 2. Iterate over all resources for this metric
        for (resource_id, series) in metric_map {
            // Create a new series for the result set
            // The key will be the tag combination (or just resource_id if no grouping)
            let series_key = if query.group_by_tags.is_empty() {
                resource_id.clone()
            } else {
                // Construct a unique key based on the tag values
                query
                    .group_by_tags
                    .iter()
                    .filter_map(|tag_key| {
                    series
                        .tags
                        .iter()
                        .find(|t| t.key.as_str() == tag_key.as_str())
                        .map(|t| t.value.clone())
                    })
                    .collect::<Vec<String>>()
                    .join("|")
            };

            let result_series = results.entry(series_key).or_insert_with(|| {
                let tags = if query.group_by_tags.is_empty() {
                    series.tags.clone()
                } else {
                    // Filter tags based on the group_by keys for the result
                    series
                        .tags
                        .iter()
                        .filter(|t| query.group_by_tags.contains(&t.key))
                        .cloned()
                        .collect()
                };
                TimeSeries::new(&query.metric_name, tags)
            });

            // 3. Scan the time range (BTreeMap allows efficient slicing)
            // We iterate through the points and only keep those within [start, end]
            for (&ts, &val) in series.points.range(query.start..=query.end) {
                result_series.add_point(ts, val);
            }
        }

        // Convert HashMap to Vec for the final response
        Ok(results.into_values().collect())
    }
}

// --- 3. Example Usage ---

fn main() {
    let db = TSDB::new();
    let now = Utc::now();

    // Simulate ingestion of 1000 data points
    for i in 0..1000 {
        let timestamp = (now - Duration::seconds(1000 - i))
            .timestamp_nanos_opt()
            .expect("timestamp out of range") as u64;   

        // Inject a "problem" between i=400 and i=600
        let is_problem = i > 400 && i < 600;
        let duty_cycle = if is_problem { 15.0 } else { 90.0 }; // GPU is sleeping!

        db.ingest(DataPoint {
            resource_id: "GPU_ID_0".into(),
            metric_name: "gpu.duty_cycle".into(),
            timestamp_unix_nano: timestamp,
            value: duty_cycle,
            tags: vec![Tag { key: "location".into(), value: "gpu_core".into() }],
        });

        db.ingest(DataPoint {
            resource_id: "GPU_ID_0".into(),
            metric_name: "gpu.temperature".into(),
            timestamp_unix_nano: timestamp,
            value: 70.0,
            tags: vec![Tag { key: "location".into(), value: "gpu_core".into() }],
        });
    }

    // Query the last 100 seconds
    let query = Query {
        metric_name: "gpu.duty_cycle".into(),
        start: (now - Duration::seconds(100)).timestamp_nanos() as u64,
        end: now.timestamp_nanos() as u64,
        group_by_tags: vec!["location".to_string()],
    };

    let results = db.query(&query).unwrap();

    for ts in results {
        println!(
            "{} ({} points, avg_val: {:.2}%)",
            ts.name,
            ts.points.len(),
            ts.points
                .values()
                .sum::<f64>()
                / (ts.points.len() as f64)
        );
        // In a real app, you'd dump this to JSON or a chart here.
    }
}

