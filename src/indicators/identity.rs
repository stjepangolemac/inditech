use num_traits::Float;
use std::fmt::Debug;

use crate::{named::Named, slider::Slider};

#[derive(Debug)]
pub struct Identity<F> {
    last: Option<F>,
}

impl<F> Identity<F>
where
    F: Float,
{
    pub fn new() -> Self {
        Self { last: None }
    }
}

impl<F> Slider<F> for Identity<F>
where
    F: Float + Debug,
{
    fn last(&self) -> Option<F> {
        self.last
    }

    fn push(&mut self, item: F) {
        self.last = Some(item)
    }
}

impl<F> Named for Identity<F> {
    fn name(&self) -> String {
        "identity".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let mut ident = Identity::new();

        ident.push(1.);
        ident.push(2.);
        ident.push(3.);

        assert_eq!(ident.last(), Some(3.));
    }

    #[test]
    fn identity_empty() {
        let ident: Identity<f32> = Identity::new();

        assert_eq!(ident.last(), None);
    }
}
