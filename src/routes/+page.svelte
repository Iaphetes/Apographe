<script lang="ts">

  import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
	import asyncDerivedStream from '../stream.js'
	import { writable } from 'svelte/store'
  import { invoke } from "@tauri-apps/api/core";

	let text = writable(`# test`);
  async function parse_markdown(document){
    return await invoke("parse_markdown", { document });  
  }
  let parsed_text = asyncDerivedStream(text, parse_markdown)

</script>


<main class="container">

  <div class="row">
    <div class="column1">
      <textarea bind:value={$text} id="MarkdownEditor"></textarea>
    </div>
    <div class="column2">
      <renderedmarkdown>

        {@html $parsed_text}
      </renderedmarkdown>
    </div>
  </div>
</main>
<!--
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</main>

-->
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
}

.container {
	width: 100%;
	height: 95vh;
	margin: 0;
}
.row {
  justify-content: center;
	height: 100%;
}
.column1{

	width: 49%;
	height: 100%;
	float: left;
  border:3px solid black;
}

.column2{

	width: 49%;
	height: 100%;
	float: right;
  border: 3px solid black;
}
  renderedmarkdown :global {
    h1 {
      text-align: left;
      color: red;
    }
  }
  .MarkdownEditor{
    color: red;
  }
	textarea {
		width: 100%;
		height: 100%;

	}

</style>
