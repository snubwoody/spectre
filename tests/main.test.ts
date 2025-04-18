import { assertEquals, assertExists } from "@std/assert";
import { Context, newContext } from "../element.ts";
import { Browser, launch } from "puppeteer";

Deno.test("Context evaluate function runs in dom", async () => {
	const browser = await launch();
	const page = await newContext(browser);

	await page.evaluate(() => {
		const button = document.createElement("button");
		button.textContent = "Hello world";
		button.id = "my-button";
		document.body.appendChild(button);
	});

	const text = await page.evaluate(() => {
		const button = document.getElementById("my-button");
		return button?.textContent;
	});

	assertEquals(text, "Hello world");

	await browser.close();
});

Deno.test("Locate an element by it's id", async () => {
	const browser = await launch();
	const page = await newContext(browser);

	await page.evaluate(() => {
		const button = document.createElement("button");
		button.textContent = "Hello world";
		button.id = "my-button";
		document.body.appendChild(button);
	});

	const btn = await page.getById("my-button", 5000);
	const element = await page.getById(
		"does-not-exist",
		5000,
	);

	assertExists(btn);
	assertEquals(element, null);
	await browser.close();
});

Deno.test("Duplicate element id's", async () => {
	const browser = await launch();
	const page = await newContext(browser);

	await page.evaluate(() => {
		const btn1 = document.createElement("button");
		const btn2 = document.createElement("button");

		btn1.textContent = "I was first";
		btn2.textContent = "Nuh uh";
		btn1.id = "my-btn";
		btn2.id = "my-btn";

		document.body.appendChild(btn1);
		document.body.appendChild(btn2);
	});

	const btn = await page.getById("my-btn", 5000);

	assertExists(btn);
	assertEquals(await btn.textContent(), "I was first");

	await browser.close();
});

Deno.test("Element text content", async () => {
	const browser = await launch();
	const page = await newContext(browser);

	await page.evaluate(() => {
		const btn1 = document.createElement("button");
		const btn2 = document.createElement("button");
		const btn3 = document.createElement("button");

		btn1.textContent = "Button 1";
		btn2.textContent = "Button 2";
		btn1.id = "btn-1";
		btn2.id = "btn-2";
		btn3.id = "btn-3";

		document.body.appendChild(btn1);
		document.body.appendChild(btn2);
		document.body.appendChild(btn3);
	});

	const btn1 = await page.getById("btn-1", 5000);
	const btn2 = await page.getById("btn-2", 5000);
	const btn3 = await page.getById("btn-3", 5000);

	assertEquals(await btn1?.textContent(), "Button 1");
	assertEquals(await btn2?.textContent(), "Button 2");
	assertEquals(await btn3?.textContent(), "");

	await browser.close();
});

Deno.test("Locate an element by it's class", async () => {
	const browser = await launch();
	const page = await newContext(browser);

	await page.evaluate(() => {
		const button = document.createElement("button");
		button.textContent = "Hello world";
		button.className = "my-button";
		document.body.appendChild(button);
	});

	const btn = await page.getByClass("my-button");
	const element = await page.getByClass("does-not-exist");

	assertExists(btn);
	assertEquals(element, []);
	await browser.close();
});

Deno.test("Multiple elements with the same class", async () => {
	const browser = await launch();
	const page = await newContext(browser);

	await page.evaluate(() => {
		const button = document.createElement("button");
		const link = document.createElement("a");

		link.textContent = "Hello";
		button.textContent = "world";
		link.className = "underline";
		button.className = "underline";

		document.body.appendChild(link);
		document.body.appendChild(button);
	});

	const elements = await page.getByClass("underline");

	// Elements are in the order they were appended
	assertEquals(elements.length, 2);
	assertEquals(await elements[0].textContent(), "Hello");
	assertEquals(await elements[1].textContent(), "world");

	await browser.close();
});

Deno.test("Add an element to the dom", async () => {
	const browser = await launch();
	const page = await newContext(browser);

	await page.createElement(
		"button",
		{
			id: "element",
		},
		"Click me",
	);

	const element = await page.getById("element");

	assertEquals(await element?.textContent(), "Click me");
	await browser.close();
});
