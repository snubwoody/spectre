use std::{mem, thread::sleep, time::Duration};

use spectre::{Browser, Result};
use tokio::time::interval;

#[spectre::test]
async fn get_by_class() -> Result<()> {
    let num: u32 = rand::random();
    let class = format!("my-class-{num}");

    let element = page.locate_by_class(&class).await?;
    assert!(element.is_none());

    let expr = format!(
        "
		let element = document.createElement('a');
		element.className = '{class}';
		document.body.appendChild(element);
	"
    );

    page.evaluate(&expr).await?;

    let element = page.locate_by_class(&class).await?;
    assert!(element.is_some());

    Ok(())
}

#[tokio::test]
async fn page_closes_when_dropped() -> Result<()> {
    let mut browser = Browser::start().await?;
    let page = browser.new_page().await?;
    let targets = browser.get_targets().await?;
    assert_eq!(targets.len(),2);
    
    mem::drop(page);
    
    // Have to wait for the page to close since it's done in a task
    let mut interval = interval(Duration::from_millis(350));
    
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;
    interval.tick().await;

    let targets = browser.get_targets().await?;
    assert_eq!(targets.len(),1);
    
    Ok(())
}
