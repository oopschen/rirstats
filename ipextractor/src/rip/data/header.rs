use chrono::Utc;


#[derive(Debug, PartialEq)]
pub struct RIPHeader<'a> {
  version: u8,
  registry: RIPRegistry,
  serial: &'a str,
  records: u32,
  start_date: Utc,
  end_date: Utc,
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
  registry: RIPRegistry,
  typ: RIPSummaryTyp,
  count: u32,
}
