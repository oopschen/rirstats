use chrono::{Date, Utc};
use super::header::{RIPRegistry, RIPSummaryTyp};


#[derive(Debug, PartialEq)]
pub struct RIPRecord<'a> {
  registry: RIPRegistry,
  cc: &'a str,
  typ: RIPSummaryTyp,
  start: &'a str,
  value: u32,
  date: Date<Utc>,
  status: &'a str,
  exts: Option<Vec<&'a str>>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn riprecord_init() {
    let record = RIPRecord {
      registry: RIPRegistry::AFRINIC,
      cc: "CN",
      typ: RIPSummaryTyp::IPV4,
      start: "10.119.128.33",
      value: 1024,
      date: Utc::today(),
      status: "allocated",
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
