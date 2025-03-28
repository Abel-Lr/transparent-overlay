const invoke = window.__TAURI__.core.invoke;

const div_input = document.querySelector("#input_wrapper");
const url_input = document.querySelector("input#url");
const launch_button = document.querySelector("button#launch");
const tooltip = document.getElementById("tooltip");
let timeout_id;

url_input.addEventListener("keyup", (e) => {
  if (e.key === "Enter") launch_button.click();
});

launch_button.addEventListener("click", () => {
  invoke("url_is_parsable", { url: url_input.value }).then((result) => {
    if (!result) {
      if (timeout_id) clearTimeout(timeout_id);

      div_input.classList.add("input-error");
      tooltip.classList.toggle("invisible", false);
      tooltip.classList.toggle("tooltip-open", true);
      timeout_id = setTimeout(() => {
        tooltip.classList.toggle("tooltip-open", false);
      }, 2000);
    } else {
      // TODO : Open Livechat window with URL url_input.value
    }
  });
});

url_input.addEventListener("focus", () => {
  // div_input.classList.toggle("input-error", false);
  tooltip.classList.toggle("tooltip-open", false);
});
