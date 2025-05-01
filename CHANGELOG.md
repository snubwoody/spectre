# Changelog

All notable changes will be tracked in this file. This isn't just a combination of
commit messages/pull requests but rather a hand curated list of notable changes.

## [Unreleased]

### Added

- Added `spectre::test` attribute macro, this macro reuses the same browser instance for every test.

### Changed

- Download chrome to `$HOME/.spectre/browsers` instead of the the current working directory.

## 0.1.0 - 2025-4-28

### Added

- Added `spectre-cli` with support for downloading chrome.
- Added `Browser` struct which starts a chrome child process for communication.
- Added `Page` struct for controlling a single browser tab.
- Added `spectre::test` macro which uses the same browser between tests (experimental).
- Added `CDPConnection` and `CDPSession` for sending and receiving messages to the browser via the Chrome DevTools protocol.

