<script>
  import { createDir, BaseDirectory } from '@tauri-apps/api/fs';
  import Greet from "$lib/Greet.svelte";
    import { start_hydrating } from 'svelte/internal';
  var godotdownload = "https://downloads.tuxfamily.org/godotengine/4.0/beta4/Godot_v4.0-beta4_win64.exe.zip";
  
  async function open()
  {
    console.log("Started installing Game")
    await createDir('gameSource', { dir: BaseDirectory.App, recursive: true });
  }
</script>

<h1>Dungeons And Dungeons</h1>


<div class="row">

  <div>
    <div class="row">
      <button on:click={open}> Start </button>
    </div>
    <p></p>
  </div>
  
</div>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
