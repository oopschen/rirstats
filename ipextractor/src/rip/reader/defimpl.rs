use std::fs::File;
use std::boxed::Box;
use csv::{ReaderBuilder, Reader, Trim, Error, StringRecord, Position};
use chrono::{Date, Utc, TimeZone};
use super::super::data::{RIPHeader, RIPRecord, RIPSummary, RIPRegistry, RIPSummaryTyp};


pub struct RIPFile<'a> {
  file_path: &'a str,
  reader: Reader<File>,
  prev_position: Option<Position>,
}

pub trait RIPReader {
  fn header(&mut self) -> Option<RIPHeader> ;

  fn next_summary(&mut self) -> Option<RIPSummary>;

  fn next_record(&mut self) -> Option<RIPRecord>;

  fn is_done(&mut self) -> bool;
}

impl<'a> RIPFile<'a> {

  fn new(file_path: &'a str) -> Result<RIPFile, Box<Error>> {
    let csv_reader = ReaderBuilder::new().delimiter(b'|')
      .has_headers(false).flexible(true)
      .trim(Trim::All).comment(Some(b'#'))
      .from_path(file_path)?;

    Ok(RIPFile {
      file_path: file_path,
      reader: csv_reader,
      prev_position: None,
    })
  }

}

impl<'a> RIPReader for RIPFile<'a> {


  fn header(&mut self) -> Option<RIPHeader> {
    let mut read_record = StringRecord::new();
    loop {
      roll_2_cur_line_if_any(self);
      set_cur_line(self);
      if let Ok(has_record) = self.reader.read_record(&mut read_record) {
        if !has_record {
          break;
        }

        if 7 != read_record.len() {
          break;
        }

        clear_cur_line(self);

        return Some(RIPHeader {
          version: parse_version(&read_record),
          registry: parse_registry(&read_record, 1),
          serial: match read_record.get(2) {
            Some(c) => c.to_string(),
            None => "".to_string(),
          },
          records: parse_u32(&read_record, 3),
          start_date: parse_record_date(&read_record, 4),
          end_date: parse_record_date(&read_record, 5),

        });

      } else {
        break;

      }

    }

    None
  }

  fn next_summary(&mut self) -> Option<RIPSummary> {
    let mut read_record = StringRecord::new();
    roll_2_cur_line_if_any(self);

    set_cur_line(self);
    if let Ok(has_record) = self.reader.read_record(&mut read_record) {
      if !has_record {
        return None;
      }

      if let Some(read_content) = read_record.get(read_record.len() - 1) {
        if "summary" != read_content.to_lowercase().as_str() {
          return None;
        }
      } else {
        return None;
      }

      clear_cur_line(self);

      return Some(
        RIPSummary {
          registry: parse_registry(&read_record, 0),
          typ: parse_summary_typ(&read_record, 2),
          count: parse_u32(&read_record, 4),
        });
    }
    None
  }

  fn next_record(&mut self) -> Option<RIPRecord> {
    let mut read_record = StringRecord::new();
    roll_2_cur_line_if_any(self);
    set_cur_line(self);

    if let Ok(has_record) = self.reader.read_record(&mut read_record) {
      if !has_record {
        return None;
      }

      if 6 > read_record.len() {
        return None
      }

      let mut exts = None;
      if 7 < read_record.len() {
        let mut tmp_exts = vec![];
        for iter_inx in 7..read_record.len() {
          tmp_exts.push(parse_string(&read_record, iter_inx));
        }

        exts = Some(tmp_exts);
      }

      clear_cur_line(self);

      return Some(
        RIPRecord {
          registry: parse_registry(&read_record, 0),
          cc: parse_string(&read_record, 1),
          typ: parse_summary_typ(&read_record, 2),
          start: parse_string(&read_record, 3),
          value: parse_u32(&read_record, 4),
          date: parse_record_date(&read_record, 5),
          status: parse_string(&read_record, 6),
          exts,
        }
      );
    }
    None
  }

  fn is_done(&mut self) -> bool {
    self.reader.is_done()
  }

}

fn set_cur_line(rip_file: &mut RIPFile) {
  rip_file.prev_position = Some(rip_file.reader.position().clone());
}


fn roll_2_cur_line_if_any(rip_file: &mut RIPFile) {
  if let Some(ref pos) = rip_file.prev_position {
    if let Ok(_) = rip_file.reader.seek(pos.clone()) {
    }
  }
  clear_cur_line(rip_file);
}

fn clear_cur_line(rip_file: &mut RIPFile) {
  rip_file.prev_position = None;
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

fn parse_registry(record: &StringRecord, inx: usize) -> RIPRegistry {
  let def_val = RIPRegistry::RIPENCC;
  match record.get(inx) {
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

fn parse_u32(record: &StringRecord, inx: usize) -> u32 {
  let def_val = 0;
  match record.get(inx) {
    Some(val) => match val.parse::<u32>() {
      Ok(parsed_val) => parsed_val,
      Err(_) => def_val,
    },

    None => def_val,
  }
}

fn parse_date(date_str: &str) -> Date<Utc> {
  let date_with_tz = date_str.to_string() + "000000";
  match Utc.datetime_from_str(&date_with_tz, "%Y%m%d%H%M%S") {
    Ok(e) => e.date(),
    Err(_) => Utc::today(),
  }
}

fn parse_record_date(record: &StringRecord, inx: usize) -> Date<Utc> {
  match record.get(inx) {
    Some(val) => parse_date(val),
    None => Utc::today(),
  }
}

fn parse_summary_typ(record: &StringRecord, inx: usize) -> RIPSummaryTyp{
  match record.get(inx) {
    Some(val) => match val.to_lowercase().as_str() {
      "ipv4" => RIPSummaryTyp::IPV4,
      "ipv6" => RIPSummaryTyp::IPV6,
      _ => RIPSummaryTyp::ASN,
    },
    None => RIPSummaryTyp::ASN,
  }
}

fn parse_string(record: &StringRecord, inx: usize) -> String {
  match record.get(inx) {
    Some(val) => val.to_string(),
    None => "".to_string(),
  }
}

#[cfg(test)]
mod tests {
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
      return;

    }

    // check summary
    let expected_count = vec![8533, 39773, 8005];
    let expected_typ= vec![RIPSummaryTyp::ASN, RIPSummaryTyp::IPV4, RIPSummaryTyp::IPV6];
    let mut inx = 0;

    while let Some(RIPSummary{
      registry, typ, count
    }) = rip_file.next_summary() {
      assert_eq!(registry, RIPRegistry::APNIC);
      assert_eq!(typ, expected_typ[inx]);
      assert_eq!(count, expected_count[inx]);
      inx += 1;
      if inx > 2 {
        break;
      }

    }
    assert_eq!(3, inx);
    assert_eq!(None, rip_file.next_summary());

    // check record
    if let Some(RIPRecord {
      registry, cc, typ, start, value, date, status, exts
    }) = rip_file.next_record() {
      let formatter = "%Y%m%d";
      assert_eq!(registry, RIPRegistry::APNIC);
      assert_eq!(cc, "JP");
      assert_eq!(typ, RIPSummaryTyp::ASN);
      assert_eq!(start, "173");
      assert_eq!(value, 1);
      assert_eq!(date.format(formatter).to_string(), "20020801");
      assert_eq!(status, "allocated");
      assert_eq!(exts, None);
    } else {
      assert!(false, "no record found, but must have");
    }

    while let Some(RIPRecord {
      registry, cc, typ, start, value, date, status, exts
    }) = rip_file.next_record() {
      let formatter = "%Y%m%d";
      if RIPSummaryTyp::IPV4 != typ {
        continue;
      }
      assert_eq!(registry, RIPRegistry::APNIC);
      assert_eq!(cc, "AU");
      assert_eq!(start, "1.0.0.0");
      assert_eq!(value, 256);
      assert_eq!(date.format(formatter).to_string(), "20110811");
      assert_eq!(status, "assigned");
      assert_eq!(exts, None);

      assert!(!rip_file.is_done());
      return;

    }

    assert!(false, "no ipv4 found");
  }
}
