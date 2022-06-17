<script lang="ts">
	export let name: string;
	let progress = 0
	let gamePresent = false
	let testvalue = ""
	import { appDir, normalize, resolve } from '@tauri-apps/api/path';
	import { createDir, BaseDirectory, writeBinaryFile, BinaryFileContents} from '@tauri-apps/api/fs';
	import { readDir } from '@tauri-apps/api/fs';
	import { fetch, Response, ResponseType } from '@tauri-apps/api/http';
	import { Command } from '@tauri-apps/api/shell'
			// With the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:


	async function downloadGame()
	{
		await createDir('game',{dir: BaseDirectory.App,recursive: true});
		await createDir('download',{dir: BaseDirectory.App,recursive: true});
		console.log("Downloading Game");
		var response = await fetch('http://46.232.248.108/Win64.zip', {
  			method: 'GET',
  			timeout: 300,
  			responseType: ResponseType.Binary});
		await writeBinaryFile('download/Win64.zip',response.data,{dir: BaseDirectory.App});
		await decompressGame();
		
	}

	async function decompressGame()
	{
		var appDirPath = await appDir()
		
		var downloadpath = await resolve(appDirPath, 'download', 'Win64.zip');
		var runpath = await resolve(appDirPath, 'game');
		console.log(downloadpath)
		console.log(runpath)
// Invoke the command
		await invoke('execCommandExtra', { invokeMessage: 'unzip', arg: downloadpath, arg2: runpath })
		
		
		//var cmd = new Command("unzip",[downloadpath,"-d",runpath]);
		console.log("Unzipped Game Files");
		gamePresent = true
	}

	async function runGame()
	{
		var appDirPath = await appDir()
		var runPath = await resolve(appDirPath, 'game', 'DungeonsAndDungeons.exe');
		
		invoke('execCommand', { invokeMessage: "powershell", arg: runPath })
	}


</script>

<main>
	<div id="controls">
	<progress value={progress} max="100" style="width: 80%;"></progress>
	<button on:click={gamePresent?runGame:downloadGame}>{gamePresent?"Run":"Download"}</button></div>
	
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	#controls {
    position: fixed;
    bottom: 0;
    width: 100%;
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