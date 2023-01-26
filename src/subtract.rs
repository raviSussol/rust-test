pub fn subtract<T>(x: T, y: T) -> T
where T: std::ops::Sub<Output = T> + std::cmp::PartialOrd
{
    if x > y {
        x - y
    } else {
        y - x
    }
}
