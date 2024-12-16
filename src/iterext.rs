pub trait IterExt: IntoIterator + Clone {
    fn pairs(self) -> impl Iterator<Item = (Self::Item, Self::Item)>;
}

impl<T: IntoIterator + Clone> IterExt for T {
    fn pairs(self) -> impl Iterator<Item = (T::Item, T::Item)> {
        self.clone().into_iter().zip(self.into_iter().skip(1))
    }
}
