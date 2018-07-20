use chrono::{Date, Utc};
use super::header::{RIPRegistry, RIPSummaryTyp};


#[derive(Debug, PartialEq)]
pub struct RIPRecord {
  pub registry: RIPRegistry,
  pub cc: String,
  pub typ: RIPSummaryTyp,
  pub start: String,
  pub value: u32,
  pub date: Date<Utc>,
  pub status: String,
  pub exts: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn riprecord_init() {
    let record = RIPRecord {
      registry: RIPRegistry::AFRINIC,
      cc: "CN".to_string(),
      typ: RIPSummaryTyp::IPV4,
      start: "10.119.128.33".to_string(),
      value: 1024,
      date: Utc::today(),
      status: "allocated".to_string(),
      exts: None
    };

    assert_eq!(RIPRegistry::AFRINIC, record.registry);
    assert_eq!("CN", record.cc);
    assert_eq!(RIPSummaryTyp::IPV4, record.typ);
    assert_eq!("10.119.128.33", record.start);
    assert_eq!(1024, record.value);
    assert_eq!("allocated", record.status);
    assert!(None == record.exts);
  }
}
