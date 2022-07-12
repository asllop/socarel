# Changelog

All notable changes to the **Socarel** project will be documented in this file.

## [0.5.0] - ?

### Added

- Iterate subtrees from any initial node using `Tree::iterators_at()`.
- Children iterator.
- New function to find paths, `Tree::find_path`.
- Trait for Tree IDs, `TreeIdentifier`, similar to NodeContent. Also added a macro (`impl_tree_id_traits!`) to implement default details.
- In-order DFS iterator.
- Error type.

### Changes

- Deprecated `Tree::find_node`.
- Some functions that returned Option now return Result.

### Impoved

- Documentation.
- Tests.

## [0.4.0] - 2021/09/24

First release. Published in [crates.rs](https://crates.io/crates/socarel)

## [Before 0.4.0]

Development versions.