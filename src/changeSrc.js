const invoke = window.__TAURI__.core.invoke;

function changeSrc(src) {
  const iframe = document.querySelector("iframe");
  iframe.setAttribute("src", src); // Set the new iframe src
}

window.addEventListener("load", () => {
  invoke("get_url").then((url) => changeSrc(url));
});
