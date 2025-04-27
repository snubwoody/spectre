use spectre::Browser;
use spectre::Result;

#[tokio::test]
async fn get_by_class() -> Result<()> {
    let num: u32 = rand::random();
    let class = format!("my-class-{num}");
    let browser = Browser::start().await?;
    let mut page = browser.new_page().await?;

    let element = page.get_by_class(&class).await?;
    assert!(element.is_none());

    let expr = format!(
        "
		let element = document.createElement('a');
		element.className = '{class}';
		document.body.appendChild(element);
	"
    );

    page.evaluate(&expr).await?;

    let element = page.get_by_class(&class).await?;
    assert!(element.is_some());

    Ok(())
}
