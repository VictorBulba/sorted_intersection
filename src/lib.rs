pub struct SortedIntersection<'a, O: Ord, I: Iterator<Item = O>> {
    iters: &'a mut [I],
}


impl<'a, O: Ord, I: Iterator<Item = O>> SortedIntersection<'a, O, I> {
    pub fn new(iters: &'a mut [I]) -> Self {
        Self{iters}
    }
}


impl<O: Ord, I: Iterator<Item = O>> Iterator for SortedIntersection<'_, O, I> {
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        let mut max = self.iters[0].next()?;
        let mut max_index = 0;
        let mut index = 1;
        while index != max_index {
            let iter = &mut self.iters[index];
            loop {
                match iter.next() {
                    Some(x) if x == max => break,
                    Some(other) if other > max => { max_index = index; max = other; break; }
                    Some(_) => continue,
                    None => return None,
                }
            }
            index = (index + 1) % self.iters.len();
        }
        Some(max)
    }
}