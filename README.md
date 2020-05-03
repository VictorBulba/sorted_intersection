# Intersection iterator over sorted iterators

Iterator with items that are contained in all
inner iterators i. e. intersection iterator

## Example

```rust
use sorted_intersection::SortedIntersection;

let numbers1 = vec![3, 6, 9, 18, 19, 21, 23, 27];
let numbers2 = vec![6, 7, 8, 9, 18];
let numbers3 = vec![5, 6, 9, 18, 25, 27];
let mut iters = [numbers1.iter(), numbers2.iter(), numbers3.iter()];

let mut intersection_iter = SortedIntersection::new(&mut iters);

assert_eq!(intersection_iter.next(), Some(&6));
assert_eq!(intersection_iter.next(), Some(&9));
assert_eq!(intersection_iter.next(), Some(&18));
assert_eq!(intersection_iter.next(), None);
```
