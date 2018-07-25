use super::reader::{RIPFile, RIPReader};
use super::data::{RIPSummaryTyp, RIPRecord, RIPSummary, RIPHeader};
use super::opt::{FilterCondition, Matcher};
use super::helper::ipv4_count_2_mask_represents;


pub fn process_rip<'a>(file_path: &'a str, conditions: Option<&'a Vec<FilterCondition>>) {
  // new reader
  // loop:
  //  read record
  //  format record
  //  output to stdout
  //
  let mut rip_file;
  match  RIPFile::new(file_path) {
    Ok(tmp_rip_file) => {
      rip_file = tmp_rip_file;
    },

    Err(e) => {
      eprintln!("Path '{}' not found or open with error: {}", file_path, e);
      return;
    },
  }


  if let Some(RIPHeader {
    version, registry, records,
    start_date, end_date, ..
  }) = rip_file.header() {
    let formatter = "%Y-%m-%d";
    println!("registry={:?}, version={}, totol records={}, range from {} to {}.", registry, version,
             records, start_date.format(formatter), end_date.format(formatter));
  }


  while let Some(RIPSummary{
    typ, count, ..
  }) = rip_file.next_summary() {
    println!("Type {:?} has {} records", typ, count);
  }

  while let Some(RIPRecord {
      cc, typ, start, value, status, ..
  }) = rip_file.next_record() {
    // filter records
    if let Some(filter_conditions) = conditions {
      let mut is_matched = false;
      for tmp_condition in filter_conditions {
        if match_fields!(tmp_condition, "cc", &cc, "status", &status) {
          is_matched = true;
          break;
        }
      }

      if !is_matched {
        continue;
      }
    }
    // end

    match typ {
      RIPSummaryTyp::IPV4 => {
        for item in ipv4_count_2_mask_represents(&start, value).as_ref() {
          println!("{}", item);
        }
      },
      RIPSummaryTyp::IPV6 => {
        println!("{}/{}", start, value);
      },
      _ => {
        if 1 < value {
          println!("{}", start);

        } else {
          if let Ok(start_num) = start.parse::<u64>() {
            println!("{}-{}", start, (start_num + (value as u64)));
          } else {
            eprintln!("parse start num {} fail", start);
            break;
          }

        }
      },
    }
  }
}
