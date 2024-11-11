use std::ops::*;

// Trait for types that have a point
pub trait HasPoint {
  type PointType;
  
  fn pt(&self) -> &Self::PointType;
}

// Trait for calculating the length (Euclidean distance)
pub trait Length {
  fn length(&self) -> f64;
}