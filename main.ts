import { launch } from "./browser.ts";

async function main() {
	const browser = await launch();
	browser.close();
}

if (import.meta.main) {
	await main();
}
