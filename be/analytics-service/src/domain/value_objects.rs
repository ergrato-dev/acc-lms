//! # Analytics Value Objects
//!
//! Value objects for the analytics domain.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

// =============================================================================
// DATE RANGE
// =============================================================================

/// Date range for queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl DateRange {
    /// Creates a new date range.
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    /// Creates a date range for the last N days.
    pub fn last_days(days: i64) -> Self {
        let end = Utc::now();
        let start = end - Duration::days(days);
        Self { start, end }
    }

    /// Creates a date range for the last N hours.
    pub fn last_hours(hours: i64) -> Self {
        let end = Utc::now();
        let start = end - Duration::hours(hours);
        Self { start, end }
    }

    /// Creates a date range for today.
    pub fn today() -> Self {
        let now = Utc::now();
        let start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let end = now.date_naive().and_hms_opt(23, 59, 59).unwrap();
        Self {
            start: DateTime::from_naive_utc_and_offset(start, Utc),
            end: DateTime::from_naive_utc_and_offset(end, Utc),
        }
    }

    /// Creates a date range for this week.
    pub fn this_week() -> Self {
        Self::last_days(7)
    }

    /// Creates a date range for this month.
    pub fn this_month() -> Self {
        Self::last_days(30)
    }

    /// Returns the duration of the range.
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    /// Checks if a timestamp is within this range.
    pub fn contains(&self, timestamp: DateTime<Utc>) -> bool {
        timestamp >= self.start && timestamp <= self.end
    }
}

// =============================================================================
// PAGINATION
// =============================================================================

/// Pagination parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 50,
            offset: 0,
        }
    }
}

impl Pagination {
    /// Creates new pagination parameters.
    pub fn new(limit: i64, offset: i64) -> Self {
        Self {
            limit: limit.min(1000).max(1),
            offset: offset.max(0),
        }
    }

    /// Returns the page number (1-indexed).
    pub fn page(&self) -> i64 {
        (self.offset / self.limit) + 1
    }
}

// =============================================================================
// AGGREGATION
// =============================================================================

/// Aggregation function type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AggregationFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
    Distinct,
}

impl std::fmt::Display for AggregationFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregationFunction::Count => write!(f, "count"),
            AggregationFunction::Sum => write!(f, "sum"),
            AggregationFunction::Avg => write!(f, "avg"),
            AggregationFunction::Min => write!(f, "min"),
            AggregationFunction::Max => write!(f, "max"),
            AggregationFunction::Distinct => write!(f, "distinct"),
        }
    }
}

// =============================================================================
// GRANULARITY
// =============================================================================

/// Time granularity for aggregations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeGranularity {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl TimeGranularity {
    /// Returns the ClickHouse interval function.
    pub fn clickhouse_interval(&self) -> &'static str {
        match self {
            TimeGranularity::Minute => "toStartOfMinute",
            TimeGranularity::Hour => "toStartOfHour",
            TimeGranularity::Day => "toStartOfDay",
            TimeGranularity::Week => "toStartOfWeek",
            TimeGranularity::Month => "toStartOfMonth",
            TimeGranularity::Year => "toStartOfYear",
        }
    }
}

impl std::fmt::Display for TimeGranularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeGranularity::Minute => write!(f, "minute"),
            TimeGranularity::Hour => write!(f, "hour"),
            TimeGranularity::Day => write!(f, "day"),
            TimeGranularity::Week => write!(f, "week"),
            TimeGranularity::Month => write!(f, "month"),
            TimeGranularity::Year => write!(f, "year"),
        }
    }
}

// =============================================================================
// FILTER
// =============================================================================

/// Filter condition for analytics queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

/// Filter operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
    IsNull,
    IsNotNull,
}

impl FilterOperator {
    /// Returns the SQL operator.
    pub fn sql_operator(&self) -> &'static str {
        match self {
            FilterOperator::Equals => "=",
            FilterOperator::NotEquals => "!=",
            FilterOperator::GreaterThan => ">",
            FilterOperator::GreaterThanOrEqual => ">=",
            FilterOperator::LessThan => "<",
            FilterOperator::LessThanOrEqual => "<=",
            FilterOperator::In => "IN",
            FilterOperator::NotIn => "NOT IN",
            FilterOperator::Contains => "LIKE",
            FilterOperator::StartsWith => "LIKE",
            FilterOperator::EndsWith => "LIKE",
            FilterOperator::IsNull => "IS NULL",
            FilterOperator::IsNotNull => "IS NOT NULL",
        }
    }
}

// =============================================================================
// SORT
// =============================================================================

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Desc
    }
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Asc => write!(f, "ASC"),
            SortDirection::Desc => write!(f, "DESC"),
        }
    }
}

/// Sort specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortSpec {
    pub field: String,
    pub direction: SortDirection,
}

impl SortSpec {
    pub fn asc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: SortDirection::Asc,
        }
    }

    pub fn desc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            direction: SortDirection::Desc,
        }
    }
}

// =============================================================================
// PERCENTAGE
// =============================================================================

/// Percentage value (0-100).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Percentage(f64);

impl Percentage {
    /// Creates a new percentage value.
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 100.0))
    }

    /// Creates a percentage from a ratio (0.0-1.0).
    pub fn from_ratio(ratio: f64) -> Self {
        Self::new(ratio * 100.0)
    }

    /// Returns the percentage value.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Returns the ratio (0.0-1.0).
    pub fn ratio(&self) -> f64 {
        self.0 / 100.0
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}%", self.0)
    }
}
