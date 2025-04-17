import { Page } from "puppeteer";
import { ElementHandle } from "puppeteer";
import {launch, Browser} from "puppeteer";


async function getAttributeNames(element: ElementHandle,page:Page):Promise<string[]>{
	const attrs = await page.evaluate(element => {
		return element.getAttributeNames()
	},element)

	return attrs
}

class HtmlElement{
	constructor(){

	}
}

async function main(){
	const browser = await launch();
	const page = await browser.newPage()
	await page.goto("http://localhost:5173")
	
	const button = await page.locator('h1').waitHandle()
	const attrs = await getAttributeNames(button,page)
	console.log(attrs)

	await browser.close()
}


if (import.meta.main) {
	await main()
}
