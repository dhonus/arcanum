import { writable } from 'svelte/store';

export let sel_elem = writable(null);
export let sel_col = writable(null);
export let y_scr = writable(null);
export let v = writable(localStorage.getItem("vim") === "true" ? true : false);
export let browser_w = writable(localStorage.getItem("browser") === "true" ? true : false);