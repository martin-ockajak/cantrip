/// Convenient functional-style methods for existing Rust standard library collections.
///
/// ### Features
///
/// - Equivalents of suitable iterator methods are added to standard library collections
/// - Additional utility methods commonly found in collection libraries are also included
/// - Transformation methods return a new collection instance instead of returning an iterator
/// - All methods consider collection instances to be immutable although some may consume them
/// - Asymptotic complexity is optimal and performance overhead is limited to new collection creation
///
/// ### Examples
///
/// ```rust
/// use cantrip::extensions::*;
///
/// let data = vec![1, 2, 3];
///
/// data.fold(0, |r, x| r + x);           // 6
///
/// data.any(|&x| x == 1);                // true
///
/// data.clone().map(|x| x + 1);          // vec![2, 3, 4]
///
/// data.clone().filter(|&x| x > 1);      // vec![2, 3]
///
/// data.clone().add(1).distinct();       // vec![1, 2, 3]
///
/// data.clone().delete(0).tail();        // vec![3]
///
/// data.clone().grouped_by(|x| x % 2);   // HashMap::from(vec![(0, vec![2]), (1, vec![1, 3])])
///
/// data.clone().partition(|&x| x > 1);   // (vec![3], vec![1, 2])
///
/// data.clone().zip(data);               // vec![(1, 1), (2, 2), (3, 3)]
/// ```
///
/// ### Methods
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
