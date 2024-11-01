#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Price {
  integral: u64,
  scalar: u64,
}

impl Price {
  pub fn new(price: f64) -> Price {
      let scalar = 1000;
      let integral = price * (scalar as f64);
      Price {
          integral: (integral as u64),
          scalar,
      }
  }

  pub fn to_f64(&self) -> f64 {
      (self.integral as f64)  / (self.scalar as f64)
  }

  pub fn gt(&self,)
}