pub trait FromRef<T> {
    fn from_ref(value: &T) -> Self;
}
