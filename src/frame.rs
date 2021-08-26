use crate::named::Named;
use crate::Slider;
use num_traits::Float;

pub struct Frame<F> {
    indicators: Vec<Box<dyn Frameable<F>>>,
}

impl<F> Frame<F>
where
    F: Float + Default,
{
    pub fn new() -> Self {
        Self { indicators: vec![] }
    }

    pub fn add(&mut self, indicator: Box<dyn Frameable<F>>) {
        self.indicators.push(indicator);
    }

    pub fn push(&mut self, value: F) {
        self.indicators
            .iter_mut()
            .for_each(|indicator| indicator.push(value));
    }

    pub fn last(&self) -> Vec<F> {
        self.indicators
            .iter()
            .map(|indicator| indicator.last().unwrap_or_default())
            .collect()
    }

    pub fn names(&self) -> Vec<String> {
        self.indicators
            .iter()
            .map(|indicator| indicator.name())
            .collect()
    }
}

pub trait Frameable<F>: Slider<F> + Named
where
    F: Float + Default,
{
}

impl<F, T> Frameable<F> for T
where
    F: Float + Default,
    T: Slider<F> + Named,
{
}
