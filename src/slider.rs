use num_traits::Float;
use std::fmt::Debug;

pub trait Slider<F>: Debug
where
    F: Float,
{
    fn last(&self) -> Option<F>;
    fn push(&mut self, item: F);
}
