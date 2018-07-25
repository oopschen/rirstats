pub struct FilterCondition {
  field_name: String,
  field_value: String,
}

pub trait Matcher {
  fn match_field(&self, field_name: &str, field_value: &str) -> bool;
}

impl Matcher for FilterCondition {
  fn match_field(&self, field_name: &str, field_value: &str) -> bool {
    self.field_name == field_name && self.field_value == field_value
  }

}

#[macro_export]
macro_rules! new_filter_condition {
  ( $( $f:expr, $v:expr),* ) => {
    {
      let mut temp_conditions = vec![];
      $(
        temp_conditions.push(FilterCondition {
          field_name: $f,
          field_value: $v,
        });
       )*
      temp_conditions
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn new_filter() {

    let conditions = new_filter_condition!(
          String::from("cc"), String::from("CN"),
          String::from("registry"), String::from("APNIC")
    );

    assert!(conditions[0].match_field("cc", "CN"));
    assert!(conditions[1].match_field("registry", "APNIC"));
  }

}
