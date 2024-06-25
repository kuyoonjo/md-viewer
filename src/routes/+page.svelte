<script lang="ts">
  import { onMount, tick } from "svelte";
  import "./index.scss";
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";
  import katex from "katex";
  import hljs from "highlight.js";
  import defaultMd from "./example.md?raw";
  import { getCurrent, type Theme } from "@tauri-apps/api/window";
  import { readTextFile } from "@tauri-apps/plugin-fs";

  let html = "";
  let nav: HTMLElement;
  let navWidth = 200;
  let theme: Theme = "light";
  let args = "";

  async function md_to_html(md: string) {
    html = await invoke("md_to_html", {
      md,
    });
  }

  function url_to_path(url: string): Promise<string> {
    return invoke("url_to_path", {
      url,
    });
  }

  function join(url: string, path: string): Promise<string> {
    return invoke("join", {
      url,
      path,
    });
  }

  function decode_base64(base64: string): Promise<string> {
    return invoke("decode_base64", {
      base64,
    });
  }

  onMount(async () => {
    const w = getCurrent();
    console.log(w.label);
    let md = defaultMd;
    let url = await decode_base64(w.label);
    if (url) {
      const path = await url_to_path(url);
      md = await readTextFile(path);
    }

    theme = (await w.theme()) || "light";
    document.body.className = theme;

    w.onThemeChanged((e) => {
      theme = e.payload;
      document.body.className = theme;
    });

    await md_to_html(md);
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

    if (url) {
      const imgs = document.querySelectorAll("img");
      for (const img of imgs.values()) {
        const src = img.getAttribute("src");
        if (src && !src.startsWith("http")) {
          const furl = await join(url, src);
          const path = await url_to_path(furl);
          img.src = convertFileSrc(path);
        }
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
  <div>{args}</div>
  <main>
    {@html html}
  </main>
</aside>

<svelte:head>
  <link rel="stylesheet" href="stackoverflow-{theme}.min.css" />
</svelte:head>
