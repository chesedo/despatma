# Despatma
[![github]](https://github.com/chesedo/despatma)&ensp;[![crates-io]](https://crates.io/crates/despatma)&ensp;[![docs-rs]](https://docs.rs/despatma)&ensp;[![workflow]](https://github.com/chesedo/despatma/actions?query=workflow%3ARust)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
[workflow]: https://img.shields.io/github/actions/workflow/status/chesedo/despatma/rust.yml?color=green&label=&labelColor=555555&logo=github%20actions&logoColor=white&style=for-the-badge

Despatma is a collection of `des`ign `pat`tern `ma`cros (`despatma`) born from a [Honours project](https://github.com/chesedo/cos-700/blob/master/Report%20-%20Final.pdf).
It aims to provide the most common implementations for design patterns at run-time.

This project is still a **work in progress**.
The end goal is to be as [Loki](http://loki-lib.sourceforge.net/) is for C++ and more if possible.
The following patterns are currently implemented:
- [abstract_factory] - with the help of [interpolate_traits] macro
- [visitor]
- [dependency_container]

Next up for investigation is:
- [ ] Decorator
- [ ] Proxy
- [ ] Adapter
- [ ] Mediator
- [ ] Observer
- [ ] A smarter Visitor

[abstract_factory]: https://docs.rs/despatma/latest/despatma/attr.abstract_factory.html
[interpolate_traits]: https://docs.rs/despatma/latest/despatma/attr.interpolate_traits.html
[visitor]: https://docs.rs/despatma/latest/despatma/macro.visitor.html
[dependency_container]: https://docs.rs/despatma/latest/despatma/attr.dependency_container.html
