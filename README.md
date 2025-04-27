# spectre-rs

Browser automation tool using CDP

## Getting started

Make sure to download the chrome browser first.

```bash
spectre-cli download chrome
```

Browser automation and testing library. Communication is done via
the [chrome devtools protocol](https://chromedevtools.github.io/devtools-protocol/)
which sends json messages back and forth. Currently only chrome is supported, but 
firefox and safari support is planned.

```rust
use spectre::{Browser,Page,Result};

#[tokio::main]
async fn main() -> Result<()>{
    let mut browser = Browser::start().await?;
    let page = browser.goto("https://www.example.com").await?;
    let url = page.url().await?;
    
    assert_eq!(&url,"https://www.example.com/");
    Ok(())
}
```
