import type { Page } from "puppeteer";
import type { ElementHandle } from "puppeteer";
import { type Browser, launch } from "puppeteer";

export class Element {
	handle: ElementHandle;

	constructor(handle: ElementHandle) {
		this.handle = handle;
	}

	textContent = () => this.handle.evaluate((h) => h.textContent);
}

export class Context {
	private page: Page;

	constructor(page: Page) {
		this.page = page;
	}

	/** Goto a specific url */
	goto = () => this.page.goto("http://localhost:5173");

	async evaluate<T>(func: () => T) {
		return await this.page.evaluate(func);
	}

	/** Create an element in the DOM */
	async createElement<
		K extends keyof HTMLElementTagNameMap,
		T,
	>(
		tagName: K,
		attrs: Record<string, string> = {},
		textContent: string | null = null,
	) {
		await this.page.evaluate(
			(tagName, attrs, textContent) => {
				const element = document.createElement(
					tagName,
				);
				for (const key in attrs) {
					element.setAttribute(key, attrs[key]);
				}
				element.textContent = textContent;
				document.body.appendChild(element);
			},
			tagName,
			attrs,
			textContent,
		);
	}

	/**
	 * Get an element by it's id.
	 *
	 * If multiple elements have the same id then the first
	 * one will be matched.
	 *
	 * @param id - The id of the element to locate
	 * @param timeout - The timeout in milliseconds
	 */
	async getById(
		id: string,
		timeout: number = 20_000,
	): Promise<Element | null> {
		try {
			const handle = await this.page.waitForSelector(
				`#${id}`,
				{
					timeout,
				},
			);
			return handle ? new Element(handle) : null;
		} catch {
			return null;
		}
	}

	/**
	 * Get an element by it's class.
	 *
	 * @param className - The class of the element to locate
	 */
	async getByClass(
		className: string,
	): Promise<Element[]> {
		const handles = await this.page.$$(`.${className}`);
		const elements = handles.map((handle) => new Element(handle));

		return elements;
	}
}

export async function newContext(
	broswer: Browser,
): Promise<Context> {
	return new Context(await broswer.newPage());
}
