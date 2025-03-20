pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
trait NumberTheory {}
impl NumberTheory for u64 {}
pub fn gcd<T: NumberTheory>(left: T, right: T) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
