import { assertEquals } from "@std/assert";
import { Context, newContext } from "../main.ts";
import { Page } from "puppeteer";
import { ElementHandle } from "puppeteer";
import { Browser, launch } from "puppeteer";

const browser = await launch();

Deno.test(async function addTest() {
	const page = await newContext(browser);
	// await page.goto("http://localhost:5173")

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
});

await browser.close();
