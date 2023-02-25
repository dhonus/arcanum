<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    //greetMsg = await invoke("greet", { name });
    console.log(await invoke("feed", { name }));
    greetMsg = "good";
  }
</script>
<input id="greet-input" placeholder="Enter a url" bind:value={name} />

<button on:click={greet}>
  Add feed
</button>
<p>{greetMsg}</p>

