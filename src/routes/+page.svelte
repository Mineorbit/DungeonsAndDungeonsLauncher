<script>
  import Greet from "$lib/Greet.svelte";

  import { invoke } from '@tauri-apps/api/tauri'

  import { createDir, BaseDirectory } from '@tauri-apps/api/fs';

  async function update() {
    const path = (await import('@tauri-apps/api/path'));
    const appDataDir = await path.appDataDir();


    console.log(appDataDir);
    await invoke('update_repo', { path:"repo" });
    // invoke('build_game');
  }

  async function start() {
  //invoke('launch_game');
   // invoke('launch_game_from_source');
  }
</script>

<h1>Welcome to Dungeons and Dungeons!</h1>




<div class="row">
  <button on:click={update}>Update</button>
  <button on:click={start}>Start</button>
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
