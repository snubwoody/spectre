async function main() {
	// FIXME handle errors
	const command = new Deno.Command("git", {
		args: [
			"clone",
			"https://chromium.googlesource.com/chromium/tools/depot_tools.git",
		],
		stdout: "piped",
	});
	const child = command.spawn();

	// Display the process output
	const { stdout } = await child.output();
	console.log(
		new TextDecoder().decode(stdout).toString(),
	);
}

if (import.meta.main) {
	await main();
}
