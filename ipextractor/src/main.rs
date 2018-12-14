#[macro_use(load_yaml)]
extern crate clap;
extern crate ripquerier;

use std::env;
use std::path::Path;
use clap::App;
use ripquerier::rip::process_rip;
use ripquerier::rip::opt::new_filter_from_vec;


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

  let input_path = Path::new(input_file);
  let mut real_file = input_file.to_string();
  if input_path.is_relative() {
    if let Ok(mut cur_dir) = env::current_dir() {
      cur_dir.push(input_file);
      real_file = match cur_dir.to_str() {
        None => {
          eprintln!("Can not found current directory");
          return;
        },
        Some(p) => p.to_string(),
      }

    } else {
      eprintln!("Can not found current directory");
      return;

    }
  }

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

  if let Some(mut record_typs) = matches.values_of("record_typ") {
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
    process_rip(&real_file, None);
  } else {
    process_rip(&real_file, Some(
      new_filter_from_vec(&filters).as_ref()
      )
    );

  }
}
