use std::iter::repeat_with;

pub fn generate_id() -> String {
  let id: String = repeat_with(fastrand::alphanumeric).take(5).collect();
  id
}