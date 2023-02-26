<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let url = "";
  let category = "";

  let feeds = [];
  let currentFeed = {
    title: "",
    description: "",
    read: [],
  }
  let feedBody = {};
  let postBody = "";
  let postTitle = "";

  let __url__ = "";
  let __guid__ = "";

  feed();

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
        currentFeed.read = feeds[i].read;
        feedBody = feeds[i].feed.items;
        __url__ = feeds[i].url;
        break;
      }
    }
  }
  async function renderPost(value){
    __guid__ = value.guid.value;
    postBody = "";
    if (value.description !== null) postBody = value.description;
    if (value.content !== null) postBody = value.content;
    if (postBody === "") postBody
            = "The post" + value.title + "has no content. Please open in the browser to view the post.";
    postTitle = value.title;
    console.log(value);
    console.log(__url__, __guid__, "mark_read");
    let __feeds__ = await invoke("mark_read", { url:__url__, guid:__guid__ });
    feeds = __feeds__;
    for (let i = 0; i < feeds.length; i++) {
      if (feeds[i].url === __url__) {
        currentFeed.read = feeds[i].read;
        break;
      }
    }
  }
</script>
<div class="layout">
  <div class="left">
    <div class="identity">
      <img src="icon.png">
      <h4>arcanum</h4>
    </div>
    <div class="adding">
      <input placeholder="Name" bind:value={name} />
      <input placeholder="https://" bind:value={url} />
        <input placeholder="Category" bind:value={category} />
      <button on:click={feed}>
        Add feed
      </button>
      <button>Update</button>
    </div>
    {#each feeds as feed}
      <div on:click={loadFeed(feed.filename)} class="feed">
        <p>{feed.feed.title}</p>
        <span class="count">
          {feed.feed.items.length - feed.read.length}
        </span>
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
      <div on:click={renderPost(value)}
           class="entry">
        {#if !currentFeed.read.includes(value.guid.value)}
          <span class="new"></span>
        {/if}
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
  div.identity{
    display: flex;
    justify-content: left;
    padding: 0.5rem;
  }
  div.identity img {
    width: 4rem;
    height: 4rem;
    margin-right: 0.5rem;
  }
</style>