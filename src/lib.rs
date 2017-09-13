
trait DataPoint<T> {
    fn open() -> T;
    fn high() -> T;
    fn low() -> T;
    fn close() -> T;

    fn volume() -> T;
}

trait Chart<T> {

    type Item: DataPoint<T>;

    fn get(back_index: u64) -> Self::Item;


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
