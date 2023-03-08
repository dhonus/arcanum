<script xmlns="http://www.w3.org/1999/html">
  import { invoke } from "@tauri-apps/api/tauri"
  import CollapsibleSection from './CollapsibleSection.svelte'

  // to get values when Adding a feed
  let url = "";
  let category = "";
  let warning = "";
  let updating = false;

  // to get values when Updating a feed
  let feeds = {};
  let currentFeed = {
    title: "",
    description: "",
    read: [],
    filename: "",
  }
  import { writable } from 'svelte/store';

  let readBuffer = writable([]);
  let readDict = writable({});

  let readBufferValue = [];
  let readDictValue = {};

  readBuffer.subscribe(value => {
    readBufferValue = value;
  });

  readDict.subscribe(value => {
      readDictValue = value;
  });

  // to get values when Rendering a post
  let feedBody = {};
  let postBody = "";
  let postTitle = "";
  let postDate = "";
  let postLink = "";

  // to get values when Marking a post as read
  let __url__ = "";
  let __guid__ = "";

  updateFeed("");

  async function feed(){
    if (url === "") {
      warning = "Please enter a valid URL!";
      return;
    }
    if (!url.startsWith("http")) {
      warning = "Please enter a valid URL starting with http!";
      return;
    }
    let __feeds__ = await invoke("feed", { url, category }).catch((e) => {
      console.log(e);
      warning = e;
    });
    console.log(__feeds__);
    feeds = __feeds__;
    url = "";
    category = "";
    warning = "";
  }
  async function loadFeed(fileName){
    for (const [key, value] of Object.entries(feeds)) {
      for (let j = 0; j < value.length; j++){
        if (value[j].filename === fileName || fileName === "") {
          currentFeed.description = value[j].feed.description;
          currentFeed.title = value[j].feed.title;
          currentFeed.read = value[j].read;
          currentFeed.filename = value[j].filename;
          feedBody = value[j].feed.items;
          __url__ = value[j].url;
          break;
        }
      }
    }
  }
  async function updateFeed(url){
    let __feeds__ = await invoke("update_feed", { url }).catch((e) => {
      console.log(e);
    });
    feeds = __feeds__;
    await loadFeed(url);
    console.log(__feeds__);
    readBuffer.set([]);
    readDict.set({});
  }

  async function readFeed(url){
    let __feeds__ = await invoke("read_feed", { url }).catch((e) => {
      console.log(e);
    });
    feeds = __feeds__;
    await loadFeed(url);
    console.log(__feeds__);
    readBuffer.set([]);
    readDict.set({});
  }

  async function updateAll(){
    updating = true;
    feeds = await invoke("update_all", {  }).catch((e) => {
      warning = e;
    });
    updating = false;
  }
  async function deleteFeed(url){
    console.log(url, "is the thing");
    let __feeds__ = await invoke("delete_feed", { url }).catch((e) => {
      console.log(e);
    });
    feeds = __feeds__;
    await loadFeed(url);
    console.log(__feeds__);
    await loadFeed("");
  }
  async function renderPost(value) {
    __guid__ = value.guid.value;
    postBody = "";
    if (value.description !== null) postBody = value.description;
    if (value.content !== null) postBody = value.content;
    if (postBody === "") postBody
            = "The post" + value.title + "has no content. Please open in the browser to view the post.";
    postTitle = value.title;
    postDate = value.pub_date;
    postLink = value.link;
    console.log(value);

    //readBuffer.push(value.guid.value);
    readBuffer.update(buffer => [...buffer, value.guid.value]);
    console.log(readBuffer)

    // here we add the feed that is being read to the readDict; this is so that the little "not read" bubble
    // is only shown if the post is not read in any of the feeds
    // better than reloading all the feeds from rust
    if (readDictValue[currentFeed.filename] === undefined) {
      readDict.update(dict => {
        if (!currentFeed.read.includes(value.guid.value)){
          dict[currentFeed.filename] = [value.guid.value];
        }
        return dict;
      });
    } else {
      readDict.update(dict => {
        if (!dict[currentFeed.filename].includes(value.guid.value) && !currentFeed.read.includes(value.guid.value)){
          dict[currentFeed.filename].push(value.guid.value);
        }
        return dict;
      });
    }

    await markPostAsRead();
  }

  async function markPostAsRead() {
    await invoke("mark_read", { url:__url__, guid:__guid__ });
  }
</script>
<div class="layout">
  <div class="left">
    <div class="identity">
      <img src="icon.png">
      <h4>Arcanum RSS</h4>
    </div>
    <div class="adding">
    <CollapsibleSection headerText="Add new feed" expanded_in=false>
      {#if warning.length > 0}<div class="warning">{warning}</div>{/if}
        <div class="entry">
          <div>
            <input placeholder="https://" bind:value={url} />
            <input placeholder="Category" bind:value={category} />
          </div>
          <button on:click={feed} class="add_button">
            <img src="/iconmonstr-plus-lined.svg"/> Add feed
          </button>
        </div>
    </CollapsibleSection>
    <div class="spinner">
        {#if updating}
            <img src="/spinner.gif"/>
        {/if}
    </div>
    <button on:click={updateAll} class="update_button">
      <img src="/iconmonstr-refresh-lined.svg"  />
      <p>Update feeds</p>
    </button>
    </div>
    {#each Object.entries(feeds) as [key, category]}
      <CollapsibleSection headerText={key} >
        {#each category as feed}
          <div on:click={loadFeed(feed.filename)} class={feed.url === __url__ ? 'feed active' : 'feed'}>
            <p>{feed.feed.title}</p>
            {#if feed.unread - (readDictValue[feed.filename] !== undefined ? readDictValue[feed.filename].length : 0) !== 0}
              <span class="count">
                {feed.unread - (readDictValue[feed.filename] !== undefined ? readDictValue[feed.filename].length : 0)}
              </span>
            {/if}
          </div>
        {/each}
      </CollapsibleSection>
    {/each}
  </div>
  <div class="center">
    {#if currentFeed.title !== ""}
      <div class="meta-head" >
        <b>{currentFeed.title}</b>
        {currentFeed.description}
      </div>
      <div class="meta-control">
        <span>
          <button class="update"
                  on:click={updateFeed(currentFeed.filename)} title="Update feed">
            <img src="/iconmonstr-refresh-lined.svg"  />
            <p>Update</p>
          </button>
          <button class="update"
                  on:click={readFeed(currentFeed.filename)} title="Mark all as read">
            <img src="/iconmonstr-eye-check-lined.svg"  />
          </button>
        </span>
        <button class="update"
                on:click={deleteFeed(currentFeed.filename)} title="Delete feed">
          <img src="/iconmonstr-trash-can-28.svg"  />
        </button>
      </div>
    {/if}
    {#each Object.entries(feedBody) as [key, value]}
      <div on:click={renderPost(value)}
           class="{value.guid.value === __guid__ ? 'entry active' : 'entry'}">
        {#if !currentFeed.read.includes(value.guid.value) && !readBufferValue.includes(value.guid.value)}
          <span class="new"></span>
        {/if}
          <div class="header">
            {value.pub_date}
          </div>
          {value.title}
      </div>
    {/each}
    <div class="end">All caught up!</div>
  </div>
  <div class="right">
    <article>
      {#if postDate !== ""}
        <div class="meta">
          <p>{postDate}</p>
          <div class="visit">
            <a href="{postLink}" title="Visit the original site">
              <img src="/iconmonstr-globe-3.svg" class="globe">
            </a>
          </div>
        </div>
      {/if}
      {#if postTitle !== ""}
        <h2 class="title">{postTitle}</h2>
      {/if}
      {@html postBody}
    </article>
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