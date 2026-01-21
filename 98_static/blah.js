async function hot_reload_check(state2) {
    try {
        last_modified = await get_last_mod_time(state2.url + "?" + Date.now());
        if (last_modified > state2.last_modified) {
            state2.last_modified = last_modified;
            await state2.reload_cb();
        }
    } catch (e) {
        console.log("error: ", e);
    }
}


async function get_last_mod_time(url) {
    let resp = await fetch(url);
    if (!resp.ok) {
        return (/* @__PURE__ */ new Date(0)).getTime();
    } else {
        let d = new Date(resp.headers.get("Last-Modified"));
        return d.getTime();
    }
}


state = {
    url: "worker.js",
    last_modified: (/* @__PURE__ */ new Date()).getTime(),
    reload_cb: async () => {
        try {
            window.gleam_worker.terminate();
        } catch (e) {
        }
        window.gleam_worker = new Worker(state.url + "?" + Date.now());
        console.log("doing a reload now");
    }
};
setInterval(async () => hot_reload_check(state), 30);


state = {
    url: "rust_" + wasm_version + "/b_w_logic_bg.wasm",
    last_modified: (/* @__PURE__ */ new Date()).getTime(),
    reload_cb: async () => {
        console.log("index.html js: reloading b_w_logic_bg.wasm");
        let msg_code2 = await get_msg_code(url_config);
        window.ffi.reload_logic(msg_code2);
    }
};
setInterval(async () => hot_reload_check(state), 30);
