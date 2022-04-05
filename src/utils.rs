pub trait CompactorVecUtils<T> {
    fn appended(&self, to_add: T) -> Vec<T>;
}

impl<T: Clone> CompactorVecUtils<T> for Vec<T> {
    fn appended(&self, to_add: T) -> Vec<T> {
        let mut temp = self.clone();
        temp.push(to_add);
        temp
    }
}
