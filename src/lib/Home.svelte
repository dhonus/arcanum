<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let feeds = [];
  let currentFeed = {
    title: "",
    description: "",
  }
  let feedBody = {};
  let postBody = "";
  let postTitle = "";

  async function feed(){
    let __feeds__ = await invoke("feed", { name });
    console.log(__feeds__);
    feeds = __feeds__;
  }
  async function loadFeed(fileName){
    for (let i = 0; i < feeds.length; i++) {
      if (feeds[i].filename === fileName) {
        currentFeed.description = feeds[i].feed.description;
        currentFeed.title = feeds[i].feed.title;
        feedBody = feeds[i].feed.items;
        break;
      }
    }
  }
  async function renderPost(value){
    postBody = "";
    if (value.description !== null) postBody = value.description;
    if (value.content !== null) postBody = value.content;
    if (postBody === "") postBody
            = "The post" + value.title + "has no content. Please open in the browser to view the post.";
    postTitle = value.title;
  }
</script>
<div class="layout">
  <div class="left">
    <div>
      <input id="greet-input" placeholder="Enter a url" bind:value={name} />
      <button on:click={feed}>
        Add feed
      </button>
      <button>Update</button>
    </div>
    {#each feeds as feed}
      <div on:click={loadFeed(feed.filename)} class="feed">
        {feed.feed.title}
      </div>
    {/each}
  </div>
  <div class="center">
    {#if currentFeed.title !== ""}
      <div class="meta-head" >
        <b>{currentFeed.title}</b>
        {currentFeed.description}
      </div>
    {/if}
    {#each Object.entries(feedBody) as [key, value]}
      <div on:click={renderPost(value)} class="entry">
          <div class="header">
            {value.pub_date}
          </div>
          {value.title}
      </div>
    {/each}
  </div>
  <div class="right">
    <h2 class="title">{postTitle}</h2>
    {@html postBody}
  </div>
</div>
<style>
  button, input {
    background: black;
    border: 1px solid #424242;
    line-height: 1.3rem;
    padding: 0.5rem;
    color: whitesmoke;
  }
  button *, input * {
    color: whitesmoke!important;
  }
</style>