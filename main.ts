import { Page } from "puppeteer";
import { ElementHandle } from "puppeteer";
import {launch, Browser} from "puppeteer";




class HtmlElement{
	element: ElementHandle
	page: Page

	constructor(element: ElementHandle,page: Page){
		this.element = element
		this.page = page
	}

	/** Get the attribute name of the html element e.g class, id, aria-label
	 *  as an array of strings
	 * @returns {string[]} An array of attribute names
	*/
	async attributeNames():Promise<string[]>{
		const attrs = await this.page.evaluate(element => {
			return element.getAttributeNames()
		},this.element)
	
		return attrs
	}

	/** Get the attribute name of the html element e.g class, id, aria-label
	 *  as an array of strings
	 * @returns {string[]} An array of attribute names
	*/
	async textContent():Promise<string>{
		const text = await this.page.evaluate(element => {
			return element.textContent
		},this.element)
	
		return text
	}
}

async function main(){
	const browser = await launch();
	const page = await browser.newPage()
	await page.goto("http://localhost:5173")
	
	const buttonHandle = await page.locator('button').waitHandle()
	const button = new HtmlElement(buttonHandle,page)
	const attrs = await button.attributeNames()
	const text = await button.textContent()
	console.log(text)

	await browser.close()
}


if (import.meta.main) {
	await main()
}
