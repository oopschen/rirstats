#[macro_use]
extern crate clap;
extern crate ripquerier;

use ripquerier::rip::process_rip;
use ripquerier::rip::opt::new_filter_from_vec;
use clap::App;


fn main() {
  let yaml = load_yaml!("command-opts.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let input_file = match matches.value_of("input") {
    Some(f) => f,
    _ => {
      println!("--input/-i must be supplied");
      return;
    },
  };

  let mut filters = vec![];

  if let Some(mut ccs) = matches.values_of("cc") {
    while let Some(item) = ccs.next() {
      filters.push("cc".to_string());
      filters.push(item.to_uppercase());
    }
  }

  if let Some(mut sts) = matches.values_of("status") {
    while let Some(item) = sts.next() {
      filters.push("status".to_string());
      filters.push(item.to_string());
    }

  } else {
      filters.push("status".to_string());
      filters.push("allocated".to_string());
  }

  if let Some(mut record_typs) = matches.values_of("record_typs") {
    while let Some(item) = record_typs.next() {
      filters.push("typ".to_string());
      filters.push(item.to_lowercase());
    }

  } else {
      filters.push("typ".to_string());
      filters.push("ipv4".to_string());
      filters.push("typ".to_string());
      filters.push("ipv6".to_string());
  }

  if 1 > filters.len() {
    process_rip(input_file, None);
  } else {
    process_rip(input_file, Some(
      new_filter_from_vec(&filters).as_ref()
      )
    );

  }
}
