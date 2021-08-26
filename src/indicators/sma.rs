use num_traits::Float;
use ringvec::RingVec;
use std::fmt::Debug;
use std::iter::Sum;

use crate::named::Named;
use crate::slider::Slider;

use super::identity::Identity;

#[derive(Debug)]
pub struct SMA<F> {
    internal: Box<dyn Slider<F>>,
    buffer: RingVec<F>,
}

impl<F> SMA<F>
where
    F: Float + Default + Debug + 'static,
{
    pub fn new(capacity: usize) -> Self {
        let internal: Box<Identity<F>> = Box::new(Identity::new());

        Self {
            internal,
            buffer: RingVec::new(capacity),
        }
    }

    pub fn wrap(capacity: usize, internal: Box<dyn Slider<F>>) -> Self {
        Self {
            internal,
            buffer: RingVec::new(capacity),
        }
    }
}

impl<F> Slider<F> for SMA<F>
where
    F: Float + Default + Sum<F> + Debug,
{
    fn last(&self) -> Option<F> {
        if self.buffer.is_empty() {
            None
        } else {
            let len_float = F::from(self.buffer.len()).unwrap();
            Some(self.buffer.iter().cloned().sum::<F>() / len_float)
        }
    }

    fn push(&mut self, item: F) {
        self.internal.push(item);

        if let Some(item) = self.internal.last() {
            self.buffer.push_force(item);
        };
    }
}

impl<F> Named for SMA<F>
where
    F: Default,
{
    fn name(&self) -> String {
        format!("sma{}", self.buffer.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sma() {
        let mut sma = SMA::new(3);

        sma.push(1.);
        sma.push(2.);
        sma.push(3.);

        assert_eq!(sma.last(), Some(2.));
    }

    #[test]
    fn sma_not_full() {
        let mut sma = SMA::new(3);

        sma.push(1.);
        sma.push(2.);

        assert_eq!(sma.last(), Some(1.5));
    }

    #[test]
    fn sma_empty() {
        let sma: SMA<f32> = SMA::new(3);

        assert_eq!(sma.last(), None);
    }

    #[test]
    fn sma_recursive() {
        let capacity = 10;
        let mut sma = SMA::wrap(capacity, Box::new(SMA::new(capacity)));

        for i in 1..=capacity {
            sma.push(i as f32);
        }

        assert_eq!(sma.last(), Some(3.25));
    }
}
