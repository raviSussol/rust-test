pub fn divide<T>(x: T, y: T) -> T
where
    T: std::ops::Div<Output = T>,
{
    x / y
}
