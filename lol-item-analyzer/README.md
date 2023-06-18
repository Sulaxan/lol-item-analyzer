# lol-item-analyzer (library)

Contains core components to fetch, analyze, and transform items. Much of how you interact with the
library is in the form of a pipeline.

## Limitations

Core functionality is in a usable state, however, the library currently does not provide the following:
- Caching (either when fetching items or caching the analyzed/transformed items on disk)
- No way to easily add commonly used transformers (future functionality)

## Running Tests

Tests can be run with `cargo test`, however, some tests contain stdout messages, so the following can
be used instead: `cargo test -- --nocapture`.

To run a specific test: `cargo test <test name or path to test>` (of course, you can include the
`-- --nocapture` to see stdout messages)
