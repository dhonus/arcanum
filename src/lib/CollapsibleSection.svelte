<script>
    // based on suggestions from:
    // Inclusive Components by Heydon Pickering https://inclusive-components.design/collapsible-sections/
    export let headerText;
    export let expanded_in;

    let expanded = !expanded_in;
</script>

<div class="collapsible">
    <h3>
        <button aria-expanded={expanded} on:click={() => expanded = !expanded}>
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
        padding: .3rem;
    }

    button {
        display: flex;
        justify-content: left;
        align-items: center;
        gap: .6rem;
        border: none;
        margin: 0;
        padding: .6rem 0.5rem;
        background: none;
        color: whitesmoke;
        border-radius: 7px;
        font-weight: bold;
        font-size: .9rem;
    }
    button:hover {
    background: rgba(255, 255, 255, 0.1);
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