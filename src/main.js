// Lors de l'utilisation du package npm de l'API Tauri :
// import { invoke } from '@tauri-apps/api/tauri'
// // Lors de l'utilisation du script global Tauri (si vous n'utilisez pas le package npm)
// // Assurez-vous de définir `build.withGlobalTauri` dans `tauri.conf.json` sur true
const invoke = window.__TAURI__.core.invoke;

function changeSrc(src) {
  const iframe = document.getElementById("iframe");
  iframe.setAttribute("src", src); // Set the new iframe src
}

let greetInputEl;
let greetMsgEl;

window.addEventListener("load", () => {
  invoke('get_url').then((url) => changeSrc(url));
});
