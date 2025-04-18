export class Browser {
	wsUrl: string;
	process: Deno.ChildProcess;

	constructor(wsUrl: string, process: Deno.ChildProcess) {
		this.process = process;
		this.wsUrl = wsUrl;
	}

	close = () => this.process.kill();
}

export async function launch() {
	const port = "3555";
	const command = new Deno.Command(
		"chrome-win64/chrome.exe",
		{
			args: [
				`--remote-debugging-port=${port}`,
				"--headless",
				"--disable-gpu", // Needed on windows
			],
			stdout: "piped",
		},
	);
	const child = command.spawn();
	const response = await fetch(`http://localhost:${port}/json/version`);

	const { webSocketDebuggerUrl } = await response.json();
	const browser = new Browser(webSocketDebuggerUrl, child);
	return browser;
}
