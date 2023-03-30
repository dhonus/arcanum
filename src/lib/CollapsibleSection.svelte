<script>
    import { onMount } from "svelte";

    export let headerText;

    let expanded;

    onMount(() => {
        if (localStorage.getItem(headerText) === null) {
            localStorage.setItem(headerText, true);
            expanded = true;
        } else {
            expanded = localStorage.getItem(headerText) === "true";
        }
    });
</script>

<div class="collapsible">
    <h3>
        <button aria-expanded={expanded} on:click={() => {
            expanded = !expanded;
            localStorage.setItem(headerText, expanded);
        }}>
            <img src="/iconmonstr-arrow-80.svg">
            {headerText}
        </button>
    </h3>

    <div class='contents' hidden={!expanded}>
        <slot></slot>
    </div>
</div>

<style>
    h3 {
        margin: 0!important;
    }
    div.collapsible {
        padding: 0 .3rem;
    }

    button {
        display: flex;
        justify-content: left;
        align-items: center;
        gap: .6rem;
        margin: .3rem 0;
        padding: .3rem 0.5rem;
        background: none;
        color: whitesmoke;
        border-radius: 7px;
        font-weight: bold;
        font-size: .8rem;
        width: 100%;
        border: 1px solid transparent;
    }
    button:hover {
        background: #2f2f2f;
        border: 1px solid #3d3d3d!important;
        box-shadow: 0 0 5px 1px #1c1c1c;
        cursor: pointer;
    }

    button img{
        transition: transform 330ms ease-in-out;
        transform: rotate(-90deg);
        width: .8rem;
        height: .8rem;
        opacity: 0.8;
    }

    button[aria-expanded="true"] img {
        transform: rotate(0deg);
        transform-origin: center;
        transition: transform 330ms ease-in-out;
    }
</style>
