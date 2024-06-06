/// Convenient extension methods for Rust standard library collections.
///
/// Enables collection manipulation in a functional style without the usual Rust boilerplate.
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
/// use cantrip::*;
///
/// # let source = vec![1, 2, 3];
/// let data = vec![1, 2, 3];
///
/// data.fold(0, |r, x| r + x);       // 6
///
/// # let data = source.clone();
/// data.filter(|&x| x > 1);          // vec![2, 3]
///
/// # let data = source.clone();
/// data.map(|x| x + 1);              // vec![2, 3, 4]
///
/// # let data = source.clone();
/// data.add(1).unique();             // vec![1, 2, 3]
///
/// # let data = source.clone();
/// data.delete_at(0).tail();         // vec![3]
///
/// # let data = source.clone();
/// data.interleave(vec![4, 5, 6]);   // vec![(1, 4, 2, 5, 3, 6)]
///
/// # let data = source.clone();
/// data.group_by(|x| x % 2);         // HashMap::from([(0, vec![2]), (1, vec![1, 3])])
/// ```
///
//// ### Methods
///
/// | Method                   | Vec, VecDeque, LinkedList | HashSet, BTreeSet, BinaryHeap | HashMap, BTreeMap | Slice |
/// |--------------------------|:-------------------------:|:-----------------------------:|:-----------------:|:-----:|
/// | *add*                    |             *             |               *               |         *         |       |
/// | *add_all*                |             *             |               *               |         *         |       |
/// | *add_all_at*             |             *             |                               |                   |       |
/// | *add_at*                 |             *             |                               |                   |       |
/// | *all*                    |             *             |               *               |         *         |   *   |
/// | *all_equal*              |             *             |               *               |         *         |   *   |
/// | *all_unique*             |             *             |                               |                   |   *   |
/// | *any*                    |             *             |               *               |         *         |   *   |
/// | *chunked*                |             *             |                               |                   |       |
/// | *chunked_by*             |             *             |                               |                   |       |
/// | *count_by*               |             *             |               *               |         *         |   *   |
/// | *cycle*                  |             *             |                               |                   |       |
/// | *delete*                 |             *             |                               |                   |       |
/// | *delete_all*             |             *             |               *               |         *         |       |
/// | *delete_at*              |             *             |               *               |         *         |       |
/// | *delete_range*           |             *             |               *               |         *         |       |
/// | *duplicates*             |             *             |                               |                   |       |
/// | *duplicates_by*          |             *             |                               |                   |       |
/// | *enumerate*              |             *             |                               |                   |       |
/// | *fill*                   |             *             |               *               |         *         |       |
/// | *fill_with*              |             *             |               *               |         *         |       |
/// | *filter*                 |             *             |               *               |         *         |       |
/// | *filter_keys*            |                           |                               |         *         |       |
/// | *filter_map*             |             *             |               *               |         *         |       |
/// | *filter_map_to*          |             *             |               *               |         *         |       |
/// | *filter_values*          |                           |                               |         *         |       |
/// | *find_map*               |             *             |               *               |         *         |   *   |
/// | *find_map_to*            |             *             |               *               |         *         |       |
/// | *find*                   |             *             |               *               |         *         |   *   |
/// | *flat_map*               |             *             |               *               |         *         |       |
/// | *flat_map_to*            |             *             |               *               |         *         |       |
/// | *flat*                   |             *             |               *               |                   |       |
/// | *fold*                   |             *             |               *               |         *         |   *   |
/// | *frequencies*            |             *             |                               |                   |       |
/// | *frequencies_by*         |             *             |                               |                   |       |
/// | *grouped_by*             |             *             |               *               |                   |       |
/// | *interleave*             |             *             |                               |                   |       |
/// | *interleave_shortest*    |             *             |                               |                   |       |
/// | *intersect*              |             *             |               *               |         *         |       |
/// | *intersperse*            |             *             |                               |                   |       |
/// | *intersperse_with*       |             *             |                               |                   |       |
/// | *init*                   |             *             |                               |                   |   *   |
/// | *join_items*             |             *             |               *               |         *         |   *   |
/// | *largest*                |             *             |               *               |                   |       |
/// | *map*                    |             *             |               *               |         *         |       |
/// | *map_to*                 |             *             |               *               |         *         |       |
/// | *map_keys*               |                           |                               |         *         |       |
/// | *map_values*             |                           |                               |         *         |       |
/// | *map_while*              |             *             |                               |                   |       |
/// | *max_by*                 |             *             |               *               |         *         |   *   |
/// | *max_by_key*             |             *             |               *               |         *         |   *   |
/// | *max_item*               |             *             |               *               |         *         |   *   |
/// | *min_by*                 |             *             |               *               |         *         |   *   |
/// | *min_by_key*             |             *             |               *               |         *         |   *   |
/// | *min_item*               |             *             |               *               |         *         |   *   |
/// | *minmax_by*              |             *             |               *               |         *         |   *   |
/// | *minmax_by_key*          |             *             |               *               |         *         |   *   |
/// | *minmax_item*            |             *             |               *               |         *         |   *   |
/// | *partition*              |             *             |               *               |         *         |       |
/// | *partition_map*          |             *             |               *               |         *         |       |
/// | *partition_map_to*       |             *             |               *               |         *         |       |
/// | *position*               |             *             |                               |                   |   *   |
/// | *positions*              |             *             |                               |                   |   *   |
/// | *product*                |             *             |               *               |                   |       |
/// | *product_keys*           |                           |                               |         *         |       |
/// | *product_values*         |                           |                               |         *         |       |
/// | *reduce*                 |             *             |               *               |         *         |   *   |
/// | *replace*                |             *             |               *               |         *         |       |
/// | *replace_all*            |             *             |               *               |         *         |       |
/// | *replace_at*             |             *             |                               |                   |       |
/// | *replace_range*          |             *             |                               |                   |       |
/// | *rev*                    |             *             |                               |                   |       |
/// | *rfind*                  |             *             |                               |                   |   *   |
/// | *rfold*                  |             *             |                               |                   |   *   |
/// | *rpad*                   |             *             |                               |                   |       |
/// | *rpad_with*              |             *             |                               |                   |       |
/// | *rposition*              |             *             |                               |                   |   *   |
/// | *rskip*                  |             *             |                               |                   |       |
/// | *rskip_while*            |             *             |                               |                   |       |
/// | *rscan*                  |             *             |                               |                   |       |
/// | *rtake*                  |             *             |                               |                   |       |
/// | *rtake_while*            |             *             |                               |                   |       |
/// | *pad*                    |             *             |                               |                   |       |
/// | *pad_with*               |             *             |                               |                   |       |
/// | *scan*                   |             *             |               *               |         *         |       |
/// | *scan_to*                |             *             |               *               |         *         |       |
/// | *skip*                   |             *             |                               |                   |       |
/// | *skip_while*             |             *             |                               |                   |   *   |
/// | *smallest*               |             *             |               *               |                   |       |
/// | *sorted*                 |             *             |                               |                   |       |
/// | *sorted_by*              |             *             |                               |                   |       |
/// | *sorted_by_cached_key*   |             *             |                               |                   |       |
/// | *sorted_by_key*          |             *             |                               |                   |       |
/// | *sorted_unstable*        |             *             |                               |                   |       |
/// | *sorted_unstable_by*     |             *             |                               |                   |       |
/// | *sorted_unstable_by_key* |             *             |                               |                   |       |
/// | *splice*                 |             *             |                               |                   |       |
/// | *step_by*                |             *             |                               |                   |       |
/// | *sum*                    |             *             |               *               |                   |       |
/// | *sum_keys*               |                           |                               |         *         |       |
/// | *sum_values*             |                           |                               |         *         |       |
/// | *tail*                   |             *             |                               |                   |   *   |
/// | *take*                   |             *             |                               |                   |       |
/// | *take_while*             |             *             |                               |                   |   *   |
/// | *unique*                 |             *             |                               |                   |       |
/// | *unique_by*              |             *             |                               |                   |       |
/// | *unit*                   |             *             |               *               |         *         |       |
/// | *unzip*                  |             *             |                               |                   |       |
/// | *windowed*               |             *             |                               |                   |       |
/// | *zip*                    |             *             |                               |                   |       |
pub mod extensions;

// FIXME - add String extensions

pub use extensions::*;
