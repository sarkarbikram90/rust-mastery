//! High-Performance Time-Series Database for AI Observability (Mock).

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

// --- 1. Core Data Structures ---

#[derive(Debug, Clone, Serialize)]
pub struct MetricPoint {
    pub timestamp: u64,
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

    /// BTreeMap keeps timestamps sorted automatically,
    /// making range scans efficient.
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
    pub resource_id: String,
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

// --- 2. Database Engine ---

pub struct TSDB {
    /// Metric name -> Resource ID -> TimeSeries
    ///
    /// Arc allows the database to be shared between threads.
    /// Mutex provides safe concurrent access.
    data: Arc<Mutex<HashMap<String, BTreeMap<String, TimeSeries>>>>,
}

impl TSDB {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Insert a raw data point.
    pub fn ingest(&self, point: DataPoint) {
        let mut data = self.data.lock().unwrap();

        let metric_map = data
            .entry(point.metric_name.clone())
            .or_default();

        let series = metric_map
            .entry(point.resource_id)
            .or_insert_with(|| {
                TimeSeries::new(
                    &point.metric_name,
                    point.tags.clone(),
                )
            });

        series.add_point(
            point.timestamp_unix_nano,
            point.value,
        );
    }

    /// Query data over a time range.
    pub fn query(
        &self,
        query: &Query,
    ) -> Result<Vec<TimeSeries>, String> {
        let data = self.data.lock().unwrap();

        let metric_map = data
            .get(&query.metric_name)
            .ok_or_else(|| {
                format!(
                    "Metric {} not found",
                    query.metric_name
                )
            })?;

        let mut results: HashMap<String, TimeSeries> =
            HashMap::new();

        for (resource_id, series) in metric_map {
            let series_key = if query.group_by_tags.is_empty() {
                resource_id.clone()
            } else {
                query
                    .group_by_tags
                    .iter()
                    .filter_map(|tag_key| {
                        series
                            .tags
                            .iter()
                            .find(|tag| tag.key == *tag_key)
                            .map(|tag| tag.value.clone())
                    })
                    .collect::<Vec<String>>()
                    .join("|")
            };

            let result_series =
                results.entry(series_key).or_insert_with(|| {
                    let tags = if query.group_by_tags.is_empty() {
                        series.tags.clone()
                    } else {
                        series
                            .tags
                            .iter()
                            .filter(|tag| {
                                query
                                    .group_by_tags
                                    .contains(&tag.key)
                            })
                            .cloned()
                            .collect()
                    };

                    TimeSeries::new(
                        &query.metric_name,
                        tags,
                    )
                });

            for (&ts, &value) in
                series.points.range(query.start..=query.end)
            {
                result_series.add_point(ts, value);
            }
        }

        Ok(results.into_values().collect())
    }
}

// --- 3. Example Usage ---

fn main() {
    let db = TSDB::new();
    let now = Utc::now();

    // Simulate ingestion of 1000 data points.
    for i in 0..1000 {
        let timestamp = (now - Duration::seconds(1000 - i))
            .timestamp_nanos_opt()
            .expect("timestamp out of range") as u64;

        // Inject a problem between i=400 and i=600.
        let is_problem = i > 400 && i < 600;

        // GPU is sleeping.
        let duty_cycle = if is_problem {
            15.0
        } else {
            90.0
        };

        db.ingest(DataPoint {
            resource_id: "GPU_ID_0".into(),
            metric_name: "gpu.duty_cycle".into(),
            timestamp_unix_nano: timestamp,
            value: duty_cycle,
            tags: vec![Tag {
                key: "location".into(),
                value: "gpu_core".into(),
            }],
        });

        db.ingest(DataPoint {
            resource_id: "GPU_ID_0".into(),
            metric_name: "gpu.temperature".into(),
            timestamp_unix_nano: timestamp,
            value: 70.0,
            tags: vec![Tag {
                key: "location".into(),
                value: "gpu_core".into(),
            }],
        });
    }

    // Query the last 100 seconds.
    let query = Query {
        metric_name: "gpu.duty_cycle".into(),
        start: (now - Duration::seconds(100))
            .timestamp_nanos_opt()
            .expect("timestamp out of range") as u64,
        end: now
            .timestamp_nanos_opt()
            .expect("timestamp out of range") as u64,
        group_by_tags: vec!["location".to_string()],
    };

    let results = db
        .query(&query)
        .expect("query failed");

    for series in results {
        let point_count = series.points.len();

        let average = if point_count == 0 {
            0.0
        } else {
            series.points.values().sum::<f64>()
                / point_count as f64
        };

        println!(
            "{} ({} points, avg_val: {:.2}%)",
            series.name,
            point_count,
            average
        );
    }
}