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

  while let Some(next_record) = rip_file.next_record() {
    // filter records
    if let Some(filter_conditions) = conditions {
      if !match_conditions(filter_conditions, &next_record) {
        continue;
      }
    }
    // end
    let RIPRecord {typ, start, value, ..} = next_record;

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

// different type using &&, same type using ||
fn match_conditions<'a>(conditions: &'a Vec<FilterCondition>, record: &'a RIPRecord) -> bool {
  // 0 cc, 1 typ, 2 status
  let mut flag_vec: Vec<Option<bool>> = vec![None; 3];

  for tmp_condition in conditions {
    match_fields!(
      tmp_condition,

      {
        or_condition(&mut flag_vec, 2, &record.status, tmp_condition);
      },

      "cc", { or_condition(&mut flag_vec, 0, &record.cc, tmp_condition); },

      "typ",
      {
        let tmp_type_str = format!("{:?}", record.typ).to_lowercase();
        or_condition(&mut flag_vec, 1, &tmp_type_str, tmp_condition);
      }
    )

  }

  for flag in flag_vec {
    if let Some(f) = flag {
      if !f {
        return false;
      }
    }
  }
  return true;
}

fn or_condition<'a>(
  dest_flag_vec: &'a mut Vec<Option<bool>>, inx: usize,
  actual_value: &'a str, condition: &'a FilterCondition
  ) -> bool {
  let match_flag = match_field_value!(condition, actual_value);

  if let Some(flag) = dest_flag_vec[inx] {
    dest_flag_vec[inx] = Some(flag || match_flag);
    return flag | match_flag;

  } else {
    dest_flag_vec[inx] = Some(match_flag);
    return match_flag;
  }

}
