

#[spectre::test]
async fn test(browser: spectre::Browser){
	dbg!("Hi");
	let page = browser.goto("h").await.unwrap();
}