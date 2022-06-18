use rand::seq::SliceRandom;
use rand::{random, thread_rng};

pub fn maybe_translate<T: ToString>(value: T, variants: Vec<T>, possibility: f64) -> String {
  if possibility < 0.0 || possibility > 1.0 {
    panic!("Invalid possibility value: {}", possibility);
  }

  if possibility > random::<f64>() {
    random_choice(variants)
  } else {
    value.to_string()
  }
}

fn random_choice<T: ToString>(variants: Vec<T>) -> String {
  let mut range = thread_rng();
  variants.choose(&mut range).unwrap().to_string()
}
