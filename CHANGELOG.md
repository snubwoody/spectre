# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### 🚀 Features

- Use different download links on different platforms
- Update `Browser::start` to use the platform specific binary path
- Add `Browser::start_on` to start the browser on a specific port
- Reuse browser in `spectre::test` macro
- Add `Page::close`
- Add method for fetching browser cookies
- Manually implement `Default` for `Cookie` to add custom appropriate fields
- Implement methods for setting browser cookies
- Add `CloseTarget` variant to `CdpMethod`

### 🐛 Bug Fixes

- Update chrome path on macos
- Fixed incorrect chrome path
- Set file permissions after downloading chrome on linux
- Fixed wrong variable in `install_chrome`
- Missing borrow in `install_chrome`
- Wait for browser to be active before sending requests
- Set execute permissions on every downloaded file
- Import unix file permissions in `install_chrome`

### 🚜 Refactor

- Download chrome to the user's home directory
- Use localhost instead of `0.0.0.0:0` when getting a port
- Split `Browser::start` into smaller, simpler methods
- Rename spectre binary to spectre-cli due to documentation mismatch
- Update debug implementation for `CDPConnection` to hide the innter tcp steam
- Use `spectre::test` on all appropriate tests
- Rename `CDP` to `Cdp`
- Change socket address from `ipv4` to `ipv6` in `get_available_port`
- Rename `CDPMessage` to `CdpMessage`

## [0.1.0] - 2025-04-29

### 🚀 Features

- Added `HtmlElement` class
- Added public result alias
- Parse command line args in utils
- Added tests for browser
- Added cdp error type
- Added `Page` type
- Added `CDPConnection` as an abstraction for sending messages
- Added session id field on cdp connection
- Added screenshot method on `Page`
- Added test module for cdp messages
- Added new `CDPResponse` struct to represent all valid responses
- Added some more tests
- Added `NavigationError`
- Added message for navigating a page
- Added macro crate
- Implemented page navigation
- Added browser tests
- New `spectre::test` macro
- New types for DOM nodes
- New method to get nodes
- More Node types
- Added method for getting nodes by name
- Got the `Page::get_by_name()` method working
- New `CDPSession` type
- Added send method for cdp sessions
- Added runtime modele
- Add navigate and evaluate methods on cdp session
- New runtime error type
- Parse exceptions as runtime error in `CDPSession::evaluate`
- `new_page` method on `Browser`
- New selector methods on `Page`
- Add `<br>` to element enum
- Derive `Clone` on `CDPMethod`
- Add get dom method on cdp session
- Added `Element` struct
- Add get_session method on Browser
- Finish implementing resolve node method
- Finish implementing `DomNode::into_element`
- Rename `Browser::launch` to `Browser::start`
- New `url()` method on `Page`
- Add query selector methods
- Add `Page::get_by_class`
- Begin implemeting `DOM.getBoxModel`
- Change `CDPSession` to return `DomNode` instead of full response body

### 🐛 Bug Fixes

- Use error type from spectre in utils
- Fixed wrong command in workflow
- Rename `AttachToTarget` fields to camelCase
- Filter out events when listening for responses
- Connect to page through web socket url instead of using session id
- Replaced incorrect import in doc test
- Make object_id field on `RemoteObject` optional
- Fix failing tests

### 🚜 Refactor

- Added custom message types
- Start browser on any available port instead of using an env variable
- Moved page into it's own module
- Seperated cdp message and method
- Replaces web socket with cdp connection in browser
- Use the new cdp response type in browser
- Moved cdp to folder
- Make all response types clone
- Change connection field on cdp session to owned instead of reference
- Update `EMPTY_PAGE`
- Removed the core crate and moved everything into the base crate
- Make browser and page modules private
- Wrap stream in an `Arc<Mutex<>>` so it can be shared
- Change from `&mut self` to `&self` on send methods for `CDPSession` and `CDPConnection`
- Delete old get_by_x methods on page
- Rename `Page::get_by_class` to `Page::locate_by_class`
- Remove impl block for `DomNode`

### 📚 Documentation

- Update README
- Documented `Browser`
- Update cargo docs
- Update README


