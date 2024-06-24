<script lang="ts">
  import { onMount, tick } from "svelte";
  import "./index.scss";
  import { invoke } from "@tauri-apps/api/core";
  import katex from "katex";
  import hljs from "highlight.js";
  import md from "./example.md?raw";
  import { app } from "@tauri-apps/api";
  import { getCurrent, type Theme } from "@tauri-apps/api/window";

  let html = "";
  let nav: HTMLElement;
  let navWidth = 200;
  let theme: Theme = "light";

  async function md_to_html() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    html = await invoke("md_to_html", {
      md,
    });
  }

  onMount(async () => {
    const w = getCurrent();
    theme = (await w.theme()) || "light";
    document.body.className = theme;

    w.onThemeChanged((e) => {
      theme = e.payload;
      document.body.className = theme;
    });
    await md_to_html();
    await tick();
    let anchors = document.querySelectorAll(
      '[id^="nav-item-"]',
    ) as NodeListOf<HTMLAnchorElement>;
    anchors.forEach((e) => {
      e.href = "#" + e.id;
      const h = e.parentElement!;
      h.removeChild(e);
      h.id = e.id;
      e.removeAttribute("id");
      e.removeAttribute("aria-hidden");
      e.innerHTML = h.innerHTML;
      e.className = h.tagName;
      nav.appendChild(e);
    });
    anchors = document.querySelectorAll("a");
    for (const a of anchors) {
      const href = a.getAttribute("href");
      if (href && href.startsWith("http")) {
        a.target = "_blank";
      }
    }
    const maths = document.querySelectorAll("[data-math-style]");
    maths.forEach((e) => {
      try {
        katex.render(e.textContent!, e as HTMLElement);
      } catch (e) {
        console.error(e);
      }
    });
    hljs.highlightAll();
  });
</script>

<nav style="width: {navWidth}px;" bind:this={nav}></nav>

<aside style="margin-left: {navWidth + 36}px;">
  <main>
    {@html html}
  </main>
</aside>

<svelte:head>
  <link rel="stylesheet" href="stackoverflow-{theme}.min.css" />
</svelte:head>
