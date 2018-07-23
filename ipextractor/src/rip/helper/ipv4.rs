use std::boxed::Box;
use std::net::Ipv4Addr;


pub fn ipv4_count_2_mask_represents(ipv4_addr: &str, count: u32) -> Box<Vec<String>> {
  // while find next one
  //  assmble ip=ip prefix/(1 << (31 - pos))
  //  set ip prefix = ip + (1 << (32 - pos))
  let mut result = vec![];
  let mut cur_one_pos = 31;
  let mut ipv4_prefix = ipv4_2_u32(ipv4_addr);

  loop {
    if let Some(one_pos) = find_next_one_pos(count, cur_one_pos) {
      if 1 > one_pos {
        result.push(u32_2_ipv4_str(ipv4_prefix));
      } else {
        result.push(u32_2_ipv4_str(ipv4_prefix) + "/" + &(32 - one_pos).to_string());
      }
      ipv4_prefix = ipv4_prefix + (1 << one_pos);
      if 1 > one_pos {
        break;
      }
      cur_one_pos = one_pos - 1;

    } else {
      break;

    }
  }
  Box::new(result)
}

pub fn ipv4_2_u32(ipv4_addr: &str) -> u32 {
  if let Ok(ipv4_struct) = ipv4_addr.parse::<Ipv4Addr>() {
    let ocs = ipv4_struct.octets();
    return (ocs[0] as u32) << 24 | (ocs[1] as u32) << 16 | (ocs[2]  as u32) << 8 | ocs[3] as u32;
  }
  0
}

pub fn u32_2_ipv4_str(ipv4_addr: u32) -> String {
  format!("{}", Ipv4Addr::from(ipv4_addr))
}

/// Find next 1 position.
/// cur_pos: current position, [0,31]
fn find_next_one_pos(num: u32, cur_pos: u8) -> Option<u8> {
  let mut i = cur_pos as i16;
  loop {
    if 0x1 == (0x1 & (num >> i)) {
      return Some(i as u8);
    }

    i -= 1;
    if 0 > i {
      break;
    }
  }

  None
}


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn ipv4_conversion() {
    let count_is_power_2 = ipv4_count_2_mask_represents("192.168.1.0", 256);
    assert_eq!(1, count_is_power_2.len());
    assert_eq!("192.168.1.0/24", count_is_power_2[0]);


    let count_is_power_2_more = ipv4_count_2_mask_represents("192.168.1.0", 312);
    assert_eq!(4, count_is_power_2_more.len());
    assert_eq!("192.168.1.0/24", count_is_power_2_more[0]);
    assert_eq!("192.168.2.0/27", count_is_power_2_more[1]);
    assert_eq!("192.168.2.32/28", count_is_power_2_more[2]);
    assert_eq!("192.168.2.48/29", count_is_power_2_more[3]);

    let count_is_power_2_not_cidr = ipv4_count_2_mask_represents("192.168.1.0", 3);
    assert_eq!(2, count_is_power_2_not_cidr.len());
    assert_eq!("192.168.1.0/31", count_is_power_2_not_cidr[0]);
    assert_eq!("192.168.1.2", count_is_power_2_not_cidr[1]);
  }

  #[test]
  fn ipv4_convertor() {
    let result = u32_2_ipv4_str(ipv4_2_u32("192.168.1.0"));
    assert_eq!(&result, "192.168.1.0")
  }

  #[test]
  fn find_next_one() {
    assert_eq!(Some(2u8), find_next_one_pos(0x5, 31));
    assert_eq!(Some(0u8), find_next_one_pos(0x5, 1));
    assert_eq!(Some(0u8), find_next_one_pos(0x1, 31));
    assert_eq!(None, find_next_one_pos(0x0, 31));
    assert_eq!(None, find_next_one_pos(256, 1));
  }

}
