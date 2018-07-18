use chrono::{Utc, Date};


#[derive(Debug, PartialEq)]
pub struct RIPHeader {
 pub version: u8,
 pub registry: RIPRegistry,
 pub serial: String,
 pub records: u32,
 pub start_date: Date<Utc>,
 pub end_date: Date<Utc>,
}

#[derive(Debug, PartialEq)]
pub enum RIPSummaryTyp {
  IPV4,
  IPV6,
  ASN,
}

#[derive(Debug, PartialEq)]
pub enum RIPRegistry {
  APNIC,
  AFRINIC,
  ARIN,
  IANA,
  LACNIC,
  RIPENCC,
}

#[derive(Debug, PartialEq)]
pub struct RIPSummary {
 pub registry: RIPRegistry,
 pub typ: RIPSummaryTyp,
 pub count: u32,
}
