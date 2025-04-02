const bubble_div = document.getElementById("bubble-info");
const emoji_warning = document.getElementById("emoji-warning");
const tooltip = document.getElementById("tooltip");
const input_url = document.getElementById("url");
const body_dim = document.querySelector("body").getBoundingClientRect();

emoji_warning.addEventListener("click", () => {
  input_url.focus();
});

window.addEventListener("load", () => {
  let emoji_dim = emoji_warning.getBoundingClientRect();
  tooltip.style.top = `-${emoji_dim.height / 2}px`;
  tooltip.style.right = `-${emoji_dim.width / 2}px`;
});
