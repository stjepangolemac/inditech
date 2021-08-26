use num_traits::Float;
use ringvec::RingVec;
use std::fmt::Debug;
use std::iter::Sum;

use crate::{named::Named, Slider, ROC};

use super::identity::Identity;

#[derive(Debug)]
pub struct RSI<F> {
    internal: Box<dyn Slider<F>>,
    roc: ROC<F>,
    roc_buffer: RingVec<F>,
    buffer: RingVec<F>,
    last_avg_gain: F,
    last_avg_loss: F,
}

impl<F> RSI<F>
where
    F: Float + Default + Debug + 'static,
{
    pub fn new(capacity: usize) -> Self {
        let internal: Box<Identity<F>> = Box::new(Identity::new());

        Self {
            internal,
            roc: ROC::new(1),
            roc_buffer: RingVec::new(capacity),
            buffer: RingVec::new(capacity),
            last_avg_gain: F::one(),
            last_avg_loss: F::one(),
        }
    }

    pub fn wrap(capacity: usize, internal: Box<dyn Slider<F>>) -> Self {
        Self {
            internal,
            roc: ROC::new(1),
            roc_buffer: RingVec::new(capacity),
            buffer: RingVec::new(capacity),
            last_avg_gain: F::one(),
            last_avg_loss: F::one(),
        }
    }
}

impl<F> Slider<F> for RSI<F>
where
    F: Float + Default + Sum<F> + std::fmt::Debug,
{
    fn last(&self) -> Option<F> {
        self.buffer.peek_newest().copied()
    }

    fn push(&mut self, item: F) {
        self.internal.push(item);
        let internal_last = self.internal.last().unwrap_or(item);

        self.roc.push(internal_last);
        self.roc_buffer.push_force(self.roc.last().unwrap());

        let gains: Vec<F> = self
            .roc_buffer
            .iter()
            .filter(|value| *value > &F::zero())
            .copied()
            .collect();
        let losses: Vec<F> = self
            .roc_buffer
            .iter()
            .filter(|value| *value < &F::zero())
            .copied()
            .collect();

        let avg_gain: F = match gains.len() {
            0 => F::epsilon(),
            l => gains.iter().copied().sum::<F>() / F::from(l).unwrap(),
        };
        let avg_loss: F = match losses.len() {
            0 => F::epsilon(),
            l => losses.iter().map(|val| val.abs()).sum::<F>() / F::from(l).unwrap(),
        };

        let capacity = F::from(self.buffer.capacity()).unwrap();
        let one_hundred = F::from(100.).unwrap();

        let avg_gain_component = self.last_avg_gain * (capacity - F::one()) + avg_gain;
        let avg_loss_component = self.last_avg_loss * (capacity - F::one()) + avg_loss;

        let new_value =
            one_hundred - (one_hundred / (F::one() + avg_gain_component / avg_loss_component));

        self.last_avg_gain = avg_gain;
        self.last_avg_loss = avg_loss;

        self.buffer.push_force(new_value);
    }
}

impl<F> Named for RSI<F>
where
    F: Default,
{
    fn name(&self) -> String {
        format!("rsi{}", self.buffer.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsi() {
        let mut rsi = RSI::new(5);

        let vals = vec![102., 112., 120., 94., 94., 50., 25., 5.];

        for val in vals {
            rsi.push(val);
            dbg!(rsi.last());
        }

        dbg!(&rsi);
    }

    #[test]
    fn zero_change() {
        let mut rsi = RSI::new(7);

        let vals = vec![100., 105.1, 105., 105.1, 105., 105.1, 105., 105.1, 105.];

        for val in vals {
            rsi.push(val);
            dbg!(rsi.last());
        }

        dbg!(&rsi);
    }

    //     #[test]
    //     fn alpha() {
    //         let rsi: RSI<f32> = RSI::new(9);
    //
    //         assert_eq!(rsi.weight, 0.2);
    //     }
    //
    //     #[test]
    //     fn rsi_not_full() {
    //         let mut rsi = rsi::new(3);
    //
    //         rsi.push(1.);
    //         rsi.push(2.);
    //
    //         assert_eq!(rsi.last(), Some(1.5));
    //     }
    //
    //     #[test]
    //     fn rsi_empty() {
    //         let rsi: rsi<f32> = rsi::new(3);
    //
    //         assert_eq!(rsi.last(), None);
    //     }
    //
    //     #[test]
    //     fn rsi_recursive() {
    //         let capacity = 10;
    //         let mut rsi = rsi::wrap(capacity, Box::new(rsi::new(capacity)));
    //
    //         for i in 1..=capacity {
    //             rsi.push(i as f32);
    //         }
    //
    //         assert_eq!(rsi.last(), Some(3.25));
    //     }
}
