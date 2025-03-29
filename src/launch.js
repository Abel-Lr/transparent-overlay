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

      div_input.classList.toggle("input-error", true);
      tooltip.classList.toggle("invisible", false);
      tooltip.classList.toggle("tooltip-open", true);
      timeout_id = setTimeout(() => {
        tooltip.classList.toggle("tooltip-open", false);
      }, 2000);
    } else {
      div_input.classList.toggle("input-error", false);
      tooltip.classList.toggle("invisible", true);
      tooltip.classList.toggle("tooltip-open", false);

      invoke("save_config", {
        config: getConfig()
      });

      invoke("open_livechat_window", {
        config: getConfig()
      }).then(() => {
        invoke("close_config_window");
      });

    }
  });
});

function getConfig() {
  return {
    url: url_input.value
  }
}

url_input.addEventListener("focus", () => {
  // div_input.classList.toggle("input-error", false);
  tooltip.classList.toggle("tooltip-open", false);
});

window.addEventListener("load", () => {
  invoke("get_config").then((config) => {
    url_input.value = config.url
  })
})