pub fn multiply<T>(x: T, y: T) -> T
where T: std::ops::Mul<Output = T>
{
    x * y
}
