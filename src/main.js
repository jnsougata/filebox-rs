const { invoke } = window.__TAURI__.tauri;

let instanceURL;

let hambargerEl;
let sideBarLeftEl;
let loginModalEl;
let loginButtonEl;
let sidebarLeftElState = false;

function findParentInstance(data) {
  const parent_app_id = "ebf93464-fbc4-4a13-ae67-85ba912dda22";
  let instances = JSON.parse(data).instances;
  let parentInstance = null;
  instances.forEach((instance) => {
    if (instance.app_id == parent_app_id) {
      parentInstance = instance;
      return;
    }
  });
  return parentInstance;
}

async function handleSideBar() {
  if (sidebarLeftElState) {
    sideBarLeftEl.style.display = "none";
    sidebarLeftElState = false;
  } else {
    sideBarLeftEl.style.display = "flex";
    sidebarLeftElState = true;
  }
}

async function createConfigFile(data) {
  await invoke("create_app_config", { data: JSON.stringify(data), path: "app.config.json" });
}

async function fileExists() {
  const exists = await invoke("file_exists", { path: "app.config.json" });
  if (exists) {
    loginModalEl.style.display = "none";
    let config = await invoke("read_app_config", { path: "app.config.json" });
    instanceURL = JSON.parse(config).url;
  } else {
    loginModalEl.style.display = "flex";
    loginButtonEl.addEventListener("click", async () => {
      let username = document.querySelector("#username").value;
      let password = document.querySelector("#password").value;
      if (!username  || !password) {
        document.querySelector("#login-msg").innerHTML = "Please enter username and password";
      } else {
        let auth = await invoke("get_auth_cookies", {username: username, password: password});
        if (auth) {
          let token = auth.match(/_cv0_a=(.*?);/)[1];
          createConfigFile({"bearer": token});
          loginModalEl.style.display = "none";
          let instances = await invoke("fetch_instances", { bearer: token });
          let parentInstance = findParentInstance(instances);
          instanceURL = parentInstance.url;
          parentInstance["bearer"] = token;
          createConfigFile(parentInstance);
          console.log(parentInstance);
          console.log(instanceURL);
        }
      }
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  hambargerEl = document.querySelector(".menu");
  sideBarLeftEl = document.querySelector(".sidebar-left");
  loginModalEl = document.querySelector(".login");
  loginButtonEl = document.querySelector("#login");
  hambargerEl.addEventListener("click", () => {handleSideBar()});
  fileExists();
});
