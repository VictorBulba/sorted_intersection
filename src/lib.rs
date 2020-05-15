#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::missing_const_for_fn)]
#![warn(missing_docs)]


//! Intersection iterator over sorted iterators


/// Iterator with items that are contained in all
/// inner iterators i. e. intersection iterator
/// 
/// # Example
/// 
/// ```
/// use sorted_intersection::SortedIntersection;
/// 
/// let numbers1 = vec![3, 6, 9, 18, 19, 21, 23, 27];
/// let numbers2 = vec![6, 7, 8, 9, 18];
/// let numbers3 = vec![5, 6, 9, 18, 25, 27];
/// let mut iters = [numbers1.iter(), numbers2.iter(), numbers3.iter()];
///
/// let mut intersection_iter = SortedIntersection::new(&mut iters);
///
/// assert_eq!(intersection_iter.next(), Some(&6));
/// assert_eq!(intersection_iter.next(), Some(&9));
/// assert_eq!(intersection_iter.next(), Some(&18));
/// assert_eq!(intersection_iter.next(), None);
/// 
/// let numbers = vec![5, 6, 9, 18, 25, 27];
/// let mut iters = [numbers.iter()];
///
/// let mut intersection_iter = SortedIntersection::new(&mut iters);
/// assert_eq!(intersection_iter.next(), Some(&5));
/// assert_eq!(intersection_iter.next(), Some(&6));
/// assert_eq!(intersection_iter.next(), Some(&9));
/// assert_eq!(intersection_iter.next(), Some(&18));
/// assert_eq!(intersection_iter.next(), Some(&25));
/// assert_eq!(intersection_iter.next(), Some(&27));
/// assert_eq!(intersection_iter.next(), None);
/// ```
/// 
pub struct SortedIntersection<'a, O: Ord, I: Iterator<Item = O>> {
    iters: &'a mut [I],
}


impl<'a, O: Ord, I: Iterator<Item = O>> SortedIntersection<'a, O, I> {
    /// Create new intersection iterator over given iterators
    #[inline]
    pub fn new(iters: &'a mut [I]) -> Self {
        Self{iters}
    }
}


impl<O: Ord, I: Iterator<Item = O>> Iterator for SortedIntersection<'_, O, I> {
    type Item = O;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut max = self.iters.first_mut()?.next()?;
        let mut max_index = 0;
        let mut index = 1;
        while index != max_index {
            let iter = match self.iters.get_mut(index) {
                Some(i) => i,
                None => return Some(max),
            };
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