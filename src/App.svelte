<script lang="ts">
	export let name: string;
	let statusButton = "Download"
	let progress = 0
	let testvalue = ""
	import { appDir } from '@tauri-apps/api/path';
	import { createDir, BaseDirectory } from '@tauri-apps/api/fs';
	import { readDir } from '@tauri-apps/api/fs';
	import { fetch } from '@tauri-apps/api/http';
	async function downloadGame()
	{
		await createDir('game',{dir: BaseDirectory.App,recursive: true});
		const response = await fetch('http://46.232.248.108/', {
  method: 'GET',
  timeout: 30,
});
	}



</script>

<main>
	<div>
	<progress value={progress} max="100"></progress>
	<button on:click={downloadGame}>{statusButton}</button></div>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>