use chrono::NaiveDate;

pub struct TimeSeriesPoint {
    pub date: NaiveDate,
    pub count: u64,
}
