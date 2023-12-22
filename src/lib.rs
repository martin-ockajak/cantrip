/// Convenient functional-style methods for existing Rust standard library collections.
///
/// # Goals
///
/// * Reduce complexity and enhance clarity or Rust code
/// * Ensure reasonably low and predictable performance cost
/// * Require minimal learning by mirroring established interfaces
///
/// # Features
///
/// * Equivalents of suitable iterator methods are added to all standard library collection data types
/// * Utility methods inspired by other libraries are also added to the same collection data types
/// * Method names are distinct from current or planned standard library collection method names
/// * All methods consider collection instances to be immutable although some consume them
/// * Transformation methods return a new collection instance instead of an iterator
///
/// # Examples
///
/// ```rust
/// use cantrip::extensions::*;
///
/// let data = vec![0, 1, 2];
///
/// data.map(|x| x + 1);                  // [1, 2, 3]: Vec<i32>
///
/// data.fold(0, |r, x| r + x);           // 3: i32
///
/// data.any(|&x| x == 0);                // true: bool
///
/// data.clone().filter(|&x| x > 0);      // [1, 2]: Vec<i32>
///
/// data.clone().add(0).distinct();       // [0, 1, 2]: Vec<i32>
///
/// data.clone().delete(0).tail();        // [2]: Vec<i32>
///
/// data.clone().grouped_by(|x| x % 2);   // {0: [0, 2], 1: [1]}: HashMap<i32, Vec<i32>>
///
/// data.clone().partition(|&x| x > 1);   // ([2], [0, 1]): (Vec<i32>, Vec<i32>)
///
/// data.clone().zip(data);               // [(0, 0), (1, 1), (2, 2)]: Vec<(i32, i32)>
/// ```
///
///
/// # Methods
///
/// | Method               | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice |
/// |----------------------|---------------------------|-------------------------------|-------------------|-------|
/// | *add*                | *                         | *                             | *                 |       |
/// | *all*                | *                         | *                             | *                 | *     |
/// | *any*                | *                         | *                             | *                 | *     |
/// | *chunked*            | *                         |                               |                   |       |
/// | *count_by*           | *                         | *                             | *                 | *     |
/// | *delete*             | *                         |                               |                   |       |
/// | *diff*               | *                         | *                             | *                 |       |
/// | *distinct*           | *                         |                               |                   |       |
/// | *distinct_by*        | *                         |                               |                   |       |
/// | *enumerate*          | *                         |                               |                   |       |
/// | *exclude*            | *                         | *                             | *                 |       |
/// | *filter*             | *                         | *                             | *                 |       |
/// | *filter_keys*        |                           |                               | *                 |       |
/// | *filter_map*         | *                         | *                             | *                 |       |
/// | *filter_values*      |                           |                               | *                 |       |
/// | *find_map*           | *                         | *                             | *                 |       |
/// | *find*               | *                         | *                             | *                 | *     |
/// | *flat_map*           | *                         | *                             | *                 |       |
/// | *flat*               | *                         | *                             |                   |       |
/// | *fold*               | *                         | *                             | *                 | *     |
/// | *grouped_by*         | *                         | *                             |                   |       |
/// | *interleave*         | *                         |                               |                   |       |
/// | *intersect*          | *                         | *                             | *                 |       |
/// | *init*               | *                         |                               |                   | *     |
/// | *map*                | *                         | *                             | *                 |       |
/// | *map_keys*           |                           |                               | *                 |       |
/// | *map_values*         |                           |                               | *                 |       |
/// | *map_while*          | *                         |                               |                   |       |
/// | *max_by*             | *                         | *                             | *                 | *     |
/// | *max_entry*          | *                         | *                             | *                 | *     |
/// | *merge*              | *                         | *                             | *                 |       |
/// | *min_by*             | *                         | *                             | *                 | *     |
/// | *min_entry*          | *                         | *                             | *                 | *     |
/// | *partition*          | *                         | *                             | *                 |       |
/// | *position*           | *                         |                               |                   | *     |
/// | *product*            | *                         | *                             |                   |       |
/// | *product_keys*       |                           |                               | *                 |       |
/// | *product_values*     |                           |                               | *                 |       |
/// | *put*                | *                         |                               |                   |       |
/// | *reduce*             | *                         | *                             | *                 | *     |
/// | *replace*            | *                         |                               |                   |       |
/// | *rev*                | *                         |                               |                   |       |
/// | *rfind*              | *                         |                               |                   | *     |
/// | *rfold*              | *                         |                               |                   | *     |
/// | *rposition*          | *                         |                               |                   | *     |
/// | *scan*               | *                         |                               |                   |       |
/// | *skip*               | *                         |                               |                   |       |
/// | *skip_while*         | *                         |                               |                   | *     |
/// | *sorted*             | *                         |                               |                   |       |
/// | *sorted_by*          | *                         |                               |                   |       |
/// | *sorted_unstable*    | *                         |                               |                   |       |
/// | *sorted_unstable_by* | *                         |                               |                   |       |
/// | *step_by*            | *                         |                               |                   |       |
/// | *sum*                | *                         | *                             |                   |       |
/// | *sum_keys*           |                           |                               | *                 |       |
/// | *sum_values*         |                           |                               | *                 |       |
/// | *tail*               | *                         |                               |                   | *     |
/// | *take*               | *                         |                               |                   |       |
/// | *take_while*         | *                         |                               |                   | *     |
/// | *unit*               | *                         | *                             | *                 |       |
/// | *unzip*              | *                         |                               |                   |       |
/// | *zip*                | *                         |                               |                   |       |
pub mod extensions;
