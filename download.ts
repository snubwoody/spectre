import {
	unZipFromFile,
} from "https://deno.land/x/zip@v1.1.0/mod.ts";

async function installChrome() {
	const response = await fetch(
		"https://storage.googleapis.com/chrome-for-testing-public/135.0.7049.95/win64/chrome-win64.zip",
	);

	if (!response.body) {
		return;
	}

	const contentLength = response.headers.get(
		"content-length",
	);
	// FIXME remove NaN
	const total = contentLength
		? parseInt(contentLength)
		: NaN;

	let count = 0;
	const reader = response.body.getReader();
	const data: Uint8Array[] = [];
	let previousPercent = 0;

	while (true) {
		const { done, value } = await reader.read();
		if (done) {
			break;
		}
		count += value.length;
		const percent = Math.round((count / total) * 100);
		if (previousPercent !== percent) {
			console.log(
				`Downloading: ${percent}%`,
			);
		}
		previousPercent = percent;
		data.push(value);
	}

	const file = new Uint8Array(count);
	let offset = 0;
	for (const chunk of data) {
		file.set(chunk, offset);
		offset += chunk.length;
	}

	Deno.writeFile("chrome-win64.zip", file);

	await unZipFromFile("chrome-win64.zip");

	Deno.remove("chrome-win64.zip");
}

async function main() {
	await installChrome();
}

if (import.meta.main) {
	await main();
}
