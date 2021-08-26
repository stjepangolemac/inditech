use num_traits::Float;
use ringvec::RingVec;
use std::fmt::Debug;
use std::iter::Sum;

use crate::named::Named;
use crate::slider::Slider;

use super::identity::Identity;

#[derive(Debug)]
pub struct EMA<F> {
    internal: Box<dyn Slider<F>>,
    buffer: RingVec<F>,
    weight: F,
}

impl<F> EMA<F>
where
    F: Float + Default + 'static + Debug,
{
    pub fn new(capacity: usize) -> Self {
        let internal: Box<Identity<F>> = Box::new(Identity::new());

        Self {
            internal,
            buffer: RingVec::new(capacity),
            weight: F::from(2.0 / (1.0 + capacity as f32)).unwrap(),
        }
    }

    pub fn wrap(capacity: usize, internal: Box<dyn Slider<F>>) -> Self {
        Self {
            internal,
            buffer: RingVec::new(capacity),
            weight: F::from(2.0 / (1.0 + capacity as f32)).unwrap(),
        }
    }
}

impl<F> Slider<F> for EMA<F>
where
    F: Float + Default + Sum<F> + std::fmt::Debug,
{
    fn last(&self) -> Option<F> {
        self.buffer.peek_newest().copied()
    }

    fn push(&mut self, item: F) {
        self.internal.push(item);
        let internal_last = self.internal.last().unwrap_or(item);

        let new_value = internal_last * self.weight
            + self.buffer.peek_newest().copied().unwrap_or(item) * (F::one() - self.weight);

        self.buffer.push_force(new_value);
    }
}

impl<F> Named for EMA<F>
where
    F: Default,
{
    fn name(&self) -> String {
        format!("ema{}", self.buffer.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ema() {
        let mut ema = EMA::new(3);

        ema.push(1.);
        ema.push(2.);
        ema.push(3.);
        ema.push(4.);
        ema.push(5.);
        ema.push(4.);
        ema.push(3.);

        dbg!(ema.last());
    }

    #[test]
    fn alpha() {
        let ema: EMA<f32> = EMA::new(9);

        assert_eq!(ema.weight, 0.2);
    }
}
