const { invoke } = window.__TAURI__.tauri;

function changeSrc(src) {
  const iframe = document.querySelector("iframe");
  iframe.setAttribute("src", src); // Set the new iframe src
}

let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {});
