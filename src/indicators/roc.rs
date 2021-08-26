use std::fmt::Debug;

use num_traits::Float;
use ringvec::RingVec;

use crate::{named::Named, slider::Slider, Identity};

#[derive(Debug)]
pub struct ROC<F> {
    internal: Box<dyn Slider<F>>,
    lookback: RingVec<F>,
    last: Option<F>,
}

impl<F> ROC<F>
where
    F: Float + Default + Debug + 'static,
{
    pub fn new(lookback: usize) -> Self {
        let internal: Box<Identity<F>> = Box::new(Identity::new());

        Self {
            internal,
            lookback: RingVec::new(lookback + 1),
            last: None,
        }
    }

    pub fn wrap(lookback: usize, internal: Box<dyn Slider<F>>) -> Self {
        Self {
            internal,
            lookback: RingVec::new(lookback + 1),
            last: None,
        }
    }
}

impl<F> Slider<F> for ROC<F>
where
    F: Float + Default + Debug,
{
    fn last(&self) -> Option<F> {
        self.last
    }

    fn push(&mut self, item: F) {
        self.internal.push(item);
        let last_item = self.internal.last().unwrap_or(item);

        self.lookback.push_force(last_item);

        let value_newest = self.lookback.peek_newest().copied().unwrap();
        let value_oldest = self.lookback.peek_oldest().copied().unwrap();

        self.last = Some((value_newest - value_oldest) / value_oldest);
    }
}

impl<F> Named for ROC<F>
where
    F: Default,
{
    fn name(&self) -> String {
        format!("roc{}", self.lookback.capacity())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roc() {
        let mut roc = ROC::new(1);

        roc.push(2.);
        dbg!(&roc);
        roc.push(3.);
        dbg!(&roc);

        assert_eq!(roc.last(), Some(0.5));
    }

    #[test]
    fn roc2() {
        let mut roc = ROC::new(1);

        roc.push(3.);
        roc.push(2.);

        dbg!(roc);
    }
}
