import { Page } from "puppeteer";
import { ElementHandle } from "puppeteer";
import { Browser, launch } from "puppeteer";

export class Context {
	private page: Page;

	constructor(page: Page) {
		this.page = page;
		// 	await page.goto("http://localhost:5173")

		// await page.evaluate(()=>{
		// 	const button = document.createElement('button')
		// 	button.textContent = "Hello world"
		// 	document.body.appendChild(button)
		// })

		// const buttonHandle = await page.locator('button').waitHandle()
		// const button = new HtmlElement(buttonHandle,page)
		// const text = await button.textContent()
		// console.log(text)

		// assertEquals(text,"Hello world")
	}
}

export async function newContext(broswer: Browser): Promise<Context> {
	return new Context(await broswer.newPage());
}

async function main() {
	const browser = await launch();
	const page = await browser.newPage();
	await page.goto("http://localhost:5173");

	await browser.close();
}

if (import.meta.main) {
	await main();
}
