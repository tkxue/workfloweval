function start_router(data) {

    console.log("start_router", data);

    var registered_ports = {};
    var msg_code = data.msg_code
    const names = ["h_index", "h_gfx", "h_vid", "ww_net", "ww_rune", "ww_python", "ww_sqlite"];

    function check_keys() {
        const lst_in = names.filter(key => key in registered_ports);
        const lst_out = names.filter(key => !(key in registered_ports));
        return {
            lst_in: lst_in,
            lst_out: lst_out,
        }
    }



    function route_msg(e) {
        let data = e.data;
        let p = registered_ports[data.to]
        if (p != null) {
            p.postMessage(data, data.transfer_list);
        } else {
            console.log("can't find process: ", data.to, data);
        }
    }

    function start_all() {
        for (var [_name, port] of Object.entries(registered_ports)) {
            port.onmessage = function (event) {
                route_msg(event);
            }
        }
        for ([_name, port] of Object.entries(registered_ports)) {
            port.postMessage({ to: _name, msg: "start" })
        }
    }

    let cnt = 0;

    function log_check_keys() {
        let x = check_keys();
        if (cnt % 100 === 0) {
            console.log("router: ports: have: ", x.lst_in, "missing: ", x.lst_out);
        }
        cnt = cnt + 1;
        if (x.lst_out.length > 0) {
            setTimeout(() => {
                log_check_keys()
            }, 10)
        } else {
            start_all();
        }
    }

    log_check_keys()

    globalThis.onmessage = function (e) {
        if (e.data.to === "router_register_port") {
            registered_ports[e.data.msg.name] = e.data.msg.port;
        } else {
            console.log("unhandled", e);
        }
    }
    globalThis.postMessage("router ready");
}

async function start_ww_net(data) {
    console.log("worker.js: start_ww_net");
    let name = "ww_net";
    let parts = make_msg_pair(name);
    let msg_code = data.msg_code;
    try {
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_logic_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_logic_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);

        module.Xdom_Logger_Util.__set_loggers__wasm("ww_net");

        let port = parts.port;
        console.log(module.G_CmsgQ.hello())
        module.G_CmsgQ.init(name,
            function (x) {
                port.postMessage(x, x.transfer_list)
            }
        );

        {
            var ws = null;
            createWS("ww_net", "ws://127.0.0.1:4000/app-api-ws",
                (e) => {
                    if (e.data === "pong") {
                    } else {
                        module.G_WsMsgQ.push_msg(e)
                    }
                },
                (ws_) => { ws = ws_; },
            )
            module.G_WsMsgQ.init((bs) => { 
                ws.send(bs) ;
            });
        }

        port.onmessage = function (x) {
            console.assert(x.data.msg === "start", `start msg is ${x.data}`);
            port.onmessage = function (x) {
                module.G_CmsgQ.push_msg(x);
            };
            (async () => {
                console.log("worker.js: rust_logic_ffi__create: ww_net: inside, rust part");
                var rust_handle = await module.Rust_Logic_Ffi.rust_logic_ffi__create("ww_net", msg_code, null);
            })();
        }
    } catch (e) {
        console.log("rust_logic_ffi: start_wasm error: ", e);
    }

    globalThis.postMessage(parts.msg, parts.msg.transfer_list);
}

async function start_ww_rune(data) {
    let parts = await rust_rune_ffi__create("ww_rune", data.msg_code);
    globalThis.postMessage(parts.msg, parts.msg.transfer_list);
}


async function start_ww_python(data) {
    let parts = await rust_python_ffi__create("ww_python", data.msg_code);
    globalThis.postMessage(parts.msg, parts.msg.transfer_list);
}

async function start_ww_sqlite(data) {
    let parts = await rust_sqlite_ffi__create("ww_sqlite", data.msg_code);
    globalThis.postMessage(parts.msg, parts.msg.transfer_list);
}

async function start_ww_sheet(data) {
    let parts = await rust_sheet_ffi__create("ww_sheet", data.msg_code);
    globalThis.postMessage(parts.msg, parts.msg.transfer_list);
}

globalThis.onmessage = function (e) {
    let data = e.data;
    switch (data.tag) {
        case "start ww_net":
            start_ww_net(data);
            break;
        case "start router":
            start_router(data);
            break;
        case "start ww_rune":
            start_ww_rune(data);
            break;
        case "start ww_python":
            start_ww_python(data);
            break;
        case "start ww_sqlite":
            start_ww_sqlite(data);
            break;
        case "start ww_sheet":
            start_ww_sheet(data);
            break;
    }
}