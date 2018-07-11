struct RIPHeader<'a>{
  version: u8,
  registry: &'a str,
  serial: &'a str,
  records: u32,
  startdate:

}
