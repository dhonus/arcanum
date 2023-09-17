<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import CollapsibleSection from "./CollapsibleSection.svelte";

  import { sel_elem, sel_col, y_scr, v, browser_w } from "./stores";

  import { onMount } from "svelte";
  let selected_element;
  let selected_column;
  let y_scroll;
  let vim = false;
  let browser;

  onMount(async () => {
    sel_elem.subscribe((value) => {
      selected_element = value;
    });
    sel_col.subscribe((value) => {
      selected_column = value;
    });

    y_scr.set(y_scroll);
    y_scr.subscribe((value) => {
      y_scroll = value;
    });

    v.subscribe((value) => {
      vim = value;
    });
    browser_w.subscribe((value) => {
      browser = value;
    });
  });

  // to get values when Adding a feed
  let url = "";
  let category = "";
  let warning = "";
  let updating = false;

  console.log(localStorage.getItem("vim"));
  console.log(localStorage.getItem("browser"));

  // to get values when Updating a feed
  let feeds = {};
  let currentFeed = {
    title: "",
    description: "",
    read: [],
    filename: "",
  };

  function vim_change(event) {
    vim = !vim;
    localStorage.setItem("vim", String(vim));
    setTimeout(() => (event.target.checked = vim), 0);
    v.set(vim);
  }
  function browser_change(event) {
    browser = !browser;
    localStorage.setItem("browser", String(browser));
    setTimeout(() => (event.target.checked = browser), 0);
    console.log(localStorage.getItem("browser"));
  }

  import { writable } from "svelte/store";

  let readBuffer = writable([]);
  let readDict = writable({});

  let readBufferValue = [];
  let readDictValue = {};

  readBuffer.subscribe((value) => {
    readBufferValue = value;
  });

  readDict.subscribe((value) => {
    readDictValue = value;
  });

  // to get values when Rendering a post
  let feedBody = {};
  let postBody = "";
  let postTitle = "";
  let postDate = "";
  let postLink = "";
  let postAuthor = "";

  // to get values when Marking a post as read
  let __url__ = "";
  let __guid__ = "";

  updateFeed("");

  async function feed() {
    if (url === "") {
      warning = "Please enter a valid URL!";
      return;
    }
    if (!url.startsWith("http")) {
      warning = "Please enter a valid URL starting with http!";
      return;
    }

    try {
      let __feeds = await invoke("feed", { url, category });
      feeds = __feeds;
    } catch (e) {
      console.log(e);
      warning = e;
      return;
    }

    url = "";
    category = "";
    warning = "";
  }
  async function loadFeed(fileName) {
    selected_column = "left";
    sel_col.set("left");
    console.log(feeds)
    // set selected element to clicked
    for (const [key, value] of Object.entries(feeds)) {
      for (let j = 0; j < value.length; j++) {
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
    // scroll up
    document.getElementById("center").scrollTop = 0;
  }
  async function updateFeed(url) {
    feeds = await invoke("update_feed", { url }).catch((e) => {
      console.log(e);
    });
    await loadFeed(url);
    readBuffer.set([]);
    readDict.set({});
  }

  async function readFeed(url) {
    feeds = await invoke("read_feed", { url }).catch((e) => {
      console.log(e);
    });
    await loadFeed(url);
    readBuffer.set([]);
    readDict.set({});
  }

  async function updateAll() {
    updating = true;
    feeds = await invoke("update_all", {}).catch((e) => {
      warning = e;
    });
    readBuffer.set([]);
    readDict.set({});
    updating = false;
  }
  async function deleteFeed(url) {
    feeds = await invoke("delete_feed", { url }).catch((e) => {
      console.log(e);
    });
    await loadFeed(url);
    await loadFeed("");
  }
  async function renderPost(event, value) {
    sel_elem.set(event.target);
    __guid__ = value.guid.value;
    postBody = "";
    if (value.description !== null) postBody = value.description;
    if (value.content !== null) {
      // this is just static HTML. We need to remove any "style" attributes from the HTML
      // because they will mess up our styling
      postBody = value.content.replace(/style="[^"]*"/g, "");
    }
    if (postBody === "")
      postBody =
        "The post" +
        value.title +
        "has no content. Please open in the browser to view the post.";
    postTitle = value.title;
    postDate = value.pub_date;
    postLink = value.link;
    postAuthor = value.author;
    if (postAuthor == undefined && value.dublin_core_ext != null) postAuthor = value.dublin_core_ext.creators[0];
    if (postAuthor == undefined && value.dublin_core != null) postAuthor = value.dublin_core.publisher;
    if (postAuthor == undefined) postAuthor = "";

    //readBuffer.push(value.guid.value);
    readBuffer.update((buffer) => [...buffer, value.guid.value]);

    // here we add the feed that is being read to the readDict; this is so that the little "not read" bubble
    // is only shown if the post is not read in any of the feeds
    // better than reloading all the feeds from rust
    if (readDictValue[currentFeed.filename] === undefined) {
      readDict.update((dict) => {
        if (!currentFeed.read.includes(value.guid.value)) {
          dict[currentFeed.filename] = [value.guid.value];
        }
        return dict;
      });
    } else {
      readDict.update((dict) => {
        if (
          !dict[currentFeed.filename].includes(value.guid.value) &&
          !currentFeed.read.includes(value.guid.value)
        ) {
          dict[currentFeed.filename].push(value.guid.value);
        }
        return dict;
      });
    }

    await markPostAsRead();
    if (selected_column !== "center") {
    }
    selected_element = document.querySelector("div.center .entry");
    sel_col.set("center");
    selected_column = "center";
  }

  async function markPostAsRead() {
    await invoke("mark_read", { url: __url__, guid: __guid__ });
  }

  // keyboard navigation

  import { on_key_down } from "./keyboard";
  import { subscribe } from "svelte/internal";
  // export the function updateAll
</script>

<svelte:window on:keydown={on_key_down} />
<div class="layout">
  <div
    class={vim && selected_column === "left" ? "left selected_column" : "left"}
  >
    <div class="identity">
      <img src="icon.png" alt="icon" />
      <h4>Arcanum RSS</h4>
    </div>
    <div class="adding">
      <CollapsibleSection headerText="Add new feed">
        {#if warning.length > 0}<div class="warning">{warning}</div>{/if}
        <div class="entry">
          <div>
            <input placeholder="https://" bind:value={url} />
            <input placeholder="Category" bind:value={category} />
          </div>
          <button on:click={feed} class="add_button">
            <img src="/iconmonstr-plus-lined.svg" alt="add feed" /> Add feed
          </button>
        </div>
      </CollapsibleSection>
      <CollapsibleSection headerText="Options">
        <div class="options">
          <div class="option">
            <label>Vim mode</label>
            <input
              type="checkbox"
              checked={vim}
              on:click|preventDefault={vim_change}
            />
          </div>
          <div class="option">
            <label>Open links in browser</label>
            <input
              type="checkbox"
              checked={browser}
              on:click|preventDefault={browser_change}
            />
          </div>
        </div>
      </CollapsibleSection>
      <div class="spinner">
        {#if updating}
          <img src="/spinner.gif" alt="spinner" />
        {/if}
      </div>
      <button on:click={updateAll} class="update_button">
        <img src="/iconmonstr-refresh-lined.svg" alt="refresh" />
        <p>Update feeds</p>
      </button>
    </div>
    <div class="list">
      {#each Object.entries(feeds) as [key, category]}
        <CollapsibleSection headerText={key}>
          {#each category as feed}
            <div
              on:click={loadFeed(feed.filename)}
              class={feed.url === __url__ ? "feed active" : "feed"}
            >
              <p>{feed.feed.title}</p>
              {#if feed.unread - (readDictValue[feed.filename] !== undefined ? readDictValue[feed.filename].length : 0) !== 0}
                <span class="count">
                  {feed.unread -
                    (readDictValue[feed.filename] !== undefined
                      ? readDictValue[feed.filename].length
                      : 0)}
                </span>
              {:else}
                <span class="count" style="opacity: 0;" />
              {/if}
            </div>
          {/each}
        </CollapsibleSection>
      {/each}
    </div>
  </div>
  <div
    class={vim && selected_column === "center"
      ? "center selected_column"
      : "center"}
    id="center"
  >
    {#if currentFeed.title !== ""}
      {#key currentFeed}
        <div transition=fadeIn class="meta-head">
          <b>{currentFeed.title}</b>
          {currentFeed.description}
        </div>
        <div transition=fadeIn class="meta-control">
          <span>
            <button
              class="update"
              on:click={updateFeed(currentFeed.filename)}
              title="Update feed"
            >
              <img src="/iconmonstr-refresh-lined.svg" alt="refresh" />
              <p>Update</p>
            </button>
            <button
              class="update"
              on:click={readFeed(currentFeed.filename)}
              title="Mark all as read"
            >
              <img src="/iconmonstr-eye-check-lined.svg" alt="mark read" />
            </button>
          </span>
          <button
            class="update"
            on:click={deleteFeed(currentFeed.filename)}
            title="Delete feed"
          >
            <img src="/iconmonstr-trash-can-28.svg" alt="delete" />
          </button>
        </div>
      {/key}
    {/if}
    {#each Object.entries(feedBody) as [key, value]}
    {#key value}
      <div transition=fadeIn
        on:click={(event) => renderPost(event, value)}
        class={value.guid.value === __guid__ ? "entry active" : "entry"}
      >
        {#if !currentFeed.read.includes(value.guid.value) && !readBufferValue.includes(value.guid.value)}
          <span class="new" />
        {/if}
        <div class="header">
          {value.pub_date}
        </div>
        {value.title}
      </div>
      {/key}
    {/each}
    <div class="end">All caught up!</div>
  </div>
    <div
      bind:this={y_scroll}
      id="right"
      class={vim && selected_column === "right"
        ? "right selected_column"
        : "right"}
    >
    {#key postBody}
      <article transition=fadeIn>
        {#if postDate !== ""}
          <div class="meta">
            <p>{postDate}</p>
            <div class="visit">
              <a href={postLink} title="Visit the original site" target={browser ? "_blank" : ""}>
                <img src="/iconmonstr-globe-3.svg" class="globe" alt="globe" />
              </a>
            </div>
          </div>
        {/if}
        {#if postAuthor !== "" && postAuthor !== null && postAuthor !== undefined}
            <div class="author"><span>by </span>{postAuthor}</div>
        {/if}
        {#if postTitle !== ""}
          <h2 class="title">{postTitle}</h2>
        {/if}
        {@html postBody}
      </article>
    {/key}
  </div>
</div>

<style>
  button,
  input {
    background: black;
    border: 1px solid #424242;
    line-height: 1.3rem;
    padding: 0.5rem;
    color: whitesmoke;
  }
  button *,
  input * {
    color: whitesmoke !important;
  }
  div.identity {
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
