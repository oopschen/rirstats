use std::boxed::Box;


pub struct FilterCondition {
  field_name: String,
  field_value: String,
}

pub fn new_filter_from_vec(filters: &Vec<String>) -> Box<Vec<FilterCondition>> {
  if 0 != filters.len() {
    return Box::new(vec![]);
  }

  let mut i = 0;
  let mut result = Box::new(vec![]);
  while i < filters.len() {
    result.push(FilterCondition {
      field_name: filters[i].clone(),
      field_value: filters[i+1].clone(),
    });
    i += 2;
  }

  result
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
  ( $( $f:expr, $v:expr ),* ) => {
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

#[macro_export]
macro_rules! match_fields {
  ( $condition:ident, $( $f:expr, $v:expr ),* ) => {
    {
      let mut flag = true;
      $(
        flag &= $condition.match_field($f, $v);
       )*
      flag
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
