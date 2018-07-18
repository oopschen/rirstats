use std::fs::File;
use std::boxed::Box;
use csv::{ReaderBuilder, Reader, Trim, Error, StringRecord};
use chrono::{Date, Utc, TimeZone};
use super::super::data::{RIPHeader, RIPRecord, RIPSummary, RIPRegistry};


pub struct RIPFile<'a> {
  file_path: &'a str,
  reader: Reader<File>,
}

pub trait RIPReader {
  fn header(&mut self) -> Option<RIPHeader> ;

  fn next_summary(&self) -> Option<RIPSummary>;

  fn next_record(&self) -> Option<RIPRecord>;
}

impl<'a> RIPFile<'a> {

  fn new(file_path: &'a str) -> Result<RIPFile, Box<Error>> {
    let csv_reader = ReaderBuilder::new().delimiter(b'|')
      .has_headers(false).flexible(true)
      .trim(Trim::All).comment(Some(b'#'))
      .from_path(file_path)?;

    Ok(RIPFile {
      file_path: file_path,
      reader: csv_reader
    })
  }

}

impl<'a> RIPReader for RIPFile<'a> {

  fn header(&mut self) -> Option<RIPHeader> {
    let mut read_record = StringRecord::new();
    loop {
      if let Ok(has_record) = self.reader.read_record(&mut read_record) {
        if !has_record {
          break;
        }

        if 7 != read_record.len() {
          continue;
        }

        return Some(RIPHeader {
          version: parse_version(&read_record),
          registry: parse_registry(&read_record),
          serial: match read_record.get(2) {
            Some(c) => c.to_string(),
            None => "".to_string(),
          },
          records: parse_records_num(&read_record),
          start_date: parse_start_date(&read_record),
          end_date: parse_end_date(&read_record),

        });

      } else {
        break;

      }

    }

    None
  }

  fn next_summary(&self) -> Option<RIPSummary> {
    None
  }

  fn next_record(&self) -> Option<RIPRecord> {
    None
  }

}

fn parse_version(record: &StringRecord) -> u8 {
  let def_val = 0;
  match record.get(0) {
    Some(c) => match c.parse::<u8>() {
      Ok(r) => r,
      Err(_) => def_val,
    },
    None => def_val,
  }
}

fn parse_registry(record: &StringRecord) -> RIPRegistry {
  let def_val = RIPRegistry::RIPENCC;
  match record.get(1) {
    Some(val) => match val.to_lowercase().as_str() {
      "apnic" => RIPRegistry::APNIC,
      "afrinic" => RIPRegistry::AFRINIC,
      "ARIN" => RIPRegistry::ARIN,
      "iana" => RIPRegistry::IANA,
      "LACNIC" => RIPRegistry::LACNIC,
      _ => def_val,
    },

    None => def_val,
  }
}

fn parse_records_num(record: &StringRecord) -> u32 {
  let def_val = 0;
  match record.get(3) {
    Some(val) => match val.parse::<u32>() {
      Ok(parsedVal) => parsedVal,
      Err(_) => def_val,
    },

    None => def_val,
  }
}

fn parse_date(date_str: &str) -> Date<Utc> {
  let date_with_tz = date_str.to_string() + "000000";
  match Utc.datetime_from_str(&date_with_tz, "%Y%m%d%H%M%S") {
    Ok(e) => e.date(),
    Err(e) => Utc::today(),
  }
}

fn parse_start_date(record: &StringRecord) -> Date<Utc> {
  match record.get(4) {
    Some(val) => parse_date(val),
    None => Utc::today(),
  }
}

fn parse_end_date(record: &StringRecord) -> Date<Utc> {
  match record.get(5) {
    Some(val) => parse_date(val),
    None => Utc::today(),
  }
}


#[cfg(test)]
mod test {
  use std::env;
  use super::*;


  #[test]
  fn reader_creation() {
    let mut path = env::current_dir().unwrap();
    path.push("src/rip/reader/delegated-apnic-latest");
    let data_file = match path.to_str() {
      None => {
        assert!(false, "file not found");
        ""
      },
      Some(p) => p,
    };

    let mut rip_file;
    if let Ok(c) = RIPFile::new(data_file) {
      rip_file = c;
    } else {
      assert!(false, "init reader fail");
      return;
    }

    if let Some(RIPHeader {
      version, registry, serial, records,
      start_date, end_date
    }) = rip_file.header() {
      let formatter = "%Y%m%d";
      assert_eq!(version, 2);
      assert_eq!(registry, RIPRegistry::APNIC);
      assert_eq!(serial, "20180717");
      assert_eq!(records, 56311);
      assert_eq!(start_date.format(formatter).to_string(), "19830613");
      assert_eq!(end_date.format(formatter).to_string(), "20180716");

    } else {
      assert!(false, "no header found");

    }

  }
}
