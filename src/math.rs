use std::{
    cmp::PartialOrd,
    ops::{Add, Div, Mul, Sub},
};

/// This is a math struct containing basic arithmetic tasks
pub struct Math;

impl Math {
    pub fn add<T>(x: T, y: T) -> T
    where
        T: Add<Output = T>,
    {
        x + y
    }

    pub fn subtract<T>(x: T, y: T) -> T
    where
        T: Sub<Output = T> + PartialOrd,
    {
        if x > y {
            x - y
        } else {
            y - x
        }
    }

    pub fn multiply<T>(x: T, y: T) -> T
    where
        T: Mul<Output = T>,
    {
        x * y
    }

    pub fn divide<T>(x: T, y: T) -> T
    where
        T: Div<Output = T>,
    {
        x / y
    }
}
