import type { event } from '@tauri-apps/api';
import { sel_elem, sel_col, y_scr, v } from './stores';
import { writable } from 'svelte/store';
import { onMount } from 'svelte';

let y_scroll: HTMLElement;
let selected_element: HTMLElement | null;
let selected_column: string | null;
let vim: boolean;

y_scr.subscribe(value => {
  y_scroll = value;
});

sel_elem.subscribe(value => {
  selected_element = value;
});

sel_col.subscribe(value => {
  selected_column = value;
});

v.subscribe(value => {
  vim = value;
});

export function on_key_down(event: KeyboardEvent) {
  console.log(y_scroll);
  // check if the user is typing in an input field
  if (document.activeElement.tagName === "INPUT") return;

  if (selected_element === null) {
    selected_element = document.querySelector("div.left .active");
    selected_element.click();
    selected_column = "left";
  }

  if (selected_element === null) {
    return;
  }

  if (event.key === "v") {
    v.set(!vim);
    return;
  }
  if (!vim) return;

  const original = selected_element;
  switch (event.key) {
    case "g": {
      switch (selected_column) {
        case "left": {
          selected_element = document.querySelector("div.left .feed");
          let first_collapsible: HTMLElement = document.querySelector("div.left div.list div.collapsible button");
          if (first_collapsible === null) break;

          if (first_collapsible.getAttribute("aria-expanded") === "false") {
            first_collapsible.click();
          }
          if (selected_element === null) break;
          selected_element.click();
          selected_element.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
          break;
        }
        case "center": {
          selected_element = document.querySelector("div.center .entry");
          if (selected_element === null) break;
          selected_element.click();
          selected_element.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
          break;
        }
        case "right": {
          if (y_scroll === null) return;
          y_scroll.scrollTo({
            top: 0,
            behavior: 'smooth'
          });
          break;
        }
      }
      break;
    }
    case "G": {
      switch (selected_column) {
        case "left": {
          selected_element = document.querySelector("div.left div.list div.collapsible:last-of-type .feed:last-of-type");
          let last_collapse: HTMLElement = document.querySelector("div.left div.list div.collapsible:last-of-type button:last-of-type");
          if (last_collapse === null) break;

          if (last_collapse.getAttribute("aria-expanded") === "false") {
            last_collapse.click();
          }
          // scroll to the bottom
          last_collapse.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
          selected_element.click();

          break;
        }
        case "center": {
          console.log("enr")
          selected_element = document.querySelector("div.center .end");
          if (selected_element === null) break;
          selected_element.previousElementSibling;
          if (selected_element === null) break;
          selected_element.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
          selected_element.click();
          break;
        }
        case "right": {
          if (y_scroll === null) return;
          y_scroll.scrollTo({
            top: y_scroll.scrollHeight,
            behavior: 'smooth'
          });
          break;
        }
      }
      break;
    }
    case "h":
      console.log("left");
      switch (selected_column) {
        case "center":
          // get all elements with class .active in div.left
          selected_element = document.querySelector("div.left .active");
          selected_column = "left";
          break;
        case "right":
          // get all elements with class .active in div.center
          selected_element = document.querySelector("div.center .active");
          selected_column = "center";
          break;
      }
      selected_element.click();
      break;
    case "j":
      switch (selected_column) {
        case "left": {
          if (selected_element.nextElementSibling === null) {
            // get the current element's parent
            selected_element = selected_element.parentElement.parentElement;
            selected_element = <HTMLElement>selected_element.nextElementSibling;
            // we skip the collapsed folders
            try {
              while (selected_element.classList.contains("collapsed")) {
                if (selected_element.nextElementSibling === null) break;
                selected_element = <HTMLElement>selected_element.nextElementSibling;
              }
              selected_element = selected_element.querySelector("div.feed");
            } catch (e) {
              selected_element = null;
              break;
            }
          }
          else {
            selected_element = <HTMLElement>selected_element.nextElementSibling;
          }
          break;
        }
        case "center": {
          selected_element = <HTMLElement>selected_element.nextElementSibling;
          break;
        }
        case "right": {
          // scroll the page down in svelte
          if (y_scroll === null) return;
          y_scroll.scrollTo({
            top: y_scroll.scrollTop + 100, // The amount to scroll down
            behavior: 'smooth' // Optional: Add smooth scrolling
          });
          return;
        }
      }
      if (selected_element === null) {
        selected_element = original;
        return;
      }
      selected_element.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
      selected_element.click();
      break;
    case "k":
      switch (selected_column) {
        case "left":
          if (selected_element.previousElementSibling === null) {
            // get the current element's parent
            selected_element = selected_element.parentElement.parentElement;
            selected_element = <HTMLElement>selected_element.previousElementSibling;
            // we skip the collapsed folders
            try {
              while (selected_element.classList.contains("collapsed")) {
                if (selected_element.previousElementSibling === null) break;
                selected_element = <HTMLElement>selected_element.previousElementSibling;
              }
              selected_element = selected_element.querySelector("div.feed");
            } catch (e) {
              selected_element = null;
            }
          }
          else {
            selected_element = <HTMLElement>selected_element.previousElementSibling;
          }
          break;
        case "center":
          selected_element = <HTMLElement>selected_element.previousElementSibling;
          break;
        case "right":
          // scroll the page down in svelte
          if (y_scroll === null) return;
          y_scroll.scrollTo({
            top: y_scroll.scrollTop - 100,
            behavior: 'smooth'
          });
          return;
      }
      if (selected_element === null) {
        selected_element = original;
        return;
      }
      selected_element.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
      selected_element.click();
      break;
    case "l":
      switch (selected_column) {
        case "left":
          // get all elements with class .active in div.center
          selected_element = document.querySelector("div.center .entry");
          if (selected_element !== null) {
            selected_element.click();
          }
          selected_column = "center";
          break;
        case "center":
          selected_element = document.querySelector("div.right");
          selected_column = "right";
          break;
      }
  }
}