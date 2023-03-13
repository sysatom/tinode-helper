const { invoke } = window.__TAURI__.tauri;

let msgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  msgEl.textContent = await invoke("greet", { name: "demo" });
}

window.addEventListener("DOMContentLoaded", () => {
  msgEl = document.querySelector("#msg");
  document
    .querySelector("#submit")
    .addEventListener("click", () => greet());
});
