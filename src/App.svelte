<script lang="ts">
  import { onMount } from 'svelte';
  import Greet from './lib/Greet.svelte'
  import { getCurrent } from '@tauri-apps/api/window';
  import { listen } from '@tauri-apps/api/event';

  const deepLinkHandlerWindows =  () => {
     console.log("listen deep linking");
  };

  const registerDeepLinkHandler = async () => {
    await getCurrent().setFocus();
    await listen("deep-link-urls", deepLinkHandlerWindows);
  };
  
  onMount(async () => {
    await getCurrent().setFocus();
    await getCurrent().center();
    await registerDeepLinkHandler();
    
  });
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Click on the Tauri, Vite, and Svelte logos to learn more.
  </p>

  <div class="row">
    <Greet />
  </div>


</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>