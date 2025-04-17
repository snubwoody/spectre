import {launch} from "puppeteer";

async function main(){
	const browser = await launch();
	const page = await browser.newPage()
	await page.goto("https://youtube.com") 
	console.log("Hello world")
}


if (import.meta.main) {
	main()
}
