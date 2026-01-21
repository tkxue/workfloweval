globalThis.my_ffi = {
    "ws": null,
    "send_msg": function (data) {
    }
}

function createWS(base_name, url, onMsg, onConnect) {
    let ws, reconnect;

    function connect() {
        ws = new WebSocket(url);
        ws.binaryType = "arraybuffer";
        globalThis.my_ffi.ws = ws;
        ws.onmessage = onMsg;
        ws.onopen = () => {
            console.log(base_name + ': ws connected');
            const pingInterval = setInterval(() => {
                if (ws.readyState === WebSocket.OPEN) {
                    ws.send(JSON.stringify({type: 'ping'}));
                } else {
                    clearInterval(pingInterval);
                }
            }, 1000);
            onConnect(ws);
        }
        ws.onclose = ws.onerror = () => {
            console.log(base_name + ': ws disconnected - reconnecting...');
            clearTimeout(reconnect);
            reconnect = setTimeout(connect, 1000); // retry every 2s
        };
    }

    connect();
    return ws;
}


function make_msg_pair(name) {
    const channel = new MessageChannel();
    const message =  // to parent to request code
        {
            to: "router_register_port",
            msg: {
                name: name,
                port: channel.port1
            },
            transfer_list: [channel.port1]
        };
    return {
        port: channel.port2,
        msg: message
    }
}


function link_msg_port(name, port, module, async_body) {
    module.G_CmsgQ.init(name,
        function (x) {
            port.postMessage(x, x.transfer_list)
        }
    );
    port.onmessage = function (x) {
        console.assert(x.data.msg === "start", `start msg is ${x.data}`);
        port.onmessage = function (x) {
            module.G_CmsgQ.push_msg(x);
        }
        async_body()
    }
}


async function rust_gfx_ffi__create(name, msg_code) {
    let parts = make_msg_pair(name);
    try {
        const port = parts.port;
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_gfx_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_gfx_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);
        module.Xdom_Logger_Util.__set_loggers__wasm(name);
        link_msg_port(name, port, module, async function () {
            var rust_handle = await module.Rust_Gfx_Ffi.rust_gfx_ffi__create(name, msg_code, null);
        });
    } catch (e) {
        console.log("rust_gfx_ffi: start_wasm error: ", e);
    }
    return parts;
}




async function rust_vid_ffi__create(name, msg_code) {
    console.log( "rust_vid_ffi__create starting rust: 000");
    let parts = make_msg_pair(name);
    try {
        const port = parts.port;
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_vid_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_vid_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);
        module.Xdom_Logger_Util.__set_loggers__wasm(name);
        link_msg_port(name, port, module, async function () {
            console.log("rust_vid_ffi__create starting rust: 111");
            var rust_handle = await module.Rust_Vid_Ffi.rust_vid_ffi__create(name, msg_code, null);
        });
    } catch (e) {
        console.log("rust_vid_ffi: start_wasm error: ", e);
    }
    return parts;
}



async function rust_logic_ffi__create(name, msg_code) {
    let parts = make_msg_pair(name);
    try {
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_logic_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_logic_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);
        module.Xdom_Logger_Util.__set_loggers__wasm(name);
        link_msg_port(name, parts.port, module, async function () {
            var rust_handle = await module.Rust_Logic_Ffi.rust_logic_ffi__create(name, msg_code, null);
        });
    } catch (e) {
        console.log("rust_logic_ffi: start_wasm error: ", e);
    }
    return parts;
}


async function rust_rune_ffi__create(name, msg_code) {
    let parts = make_msg_pair(name);
    try {
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_rune_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_rune_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);
        module.Xdom_Logger_Util.__set_loggers__wasm(name);
        link_msg_port(name, parts.port, module, async function () {
            var rust_handle = await module.Rust_Rune_Ffi.rust_rune_ffi__create(name, msg_code, null);
        });
    } catch (e) {
        console.log("rust_rune_ffi: start_wasm error: ", e);
    }
    return parts;
}



async function rust_python_ffi__create(name, msg_code) {
    let parts = make_msg_pair(name);
    try {
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_python_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_python_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);
        module.Xdom_Logger_Util.__set_loggers__wasm(name);
        link_msg_port(name, parts.port, module, async function () {
            var rust_handle = await module.Rust_Python_Ffi.rust_python_ffi__create(name, msg_code, null);
        });
    } catch (e) {
        console.log("rust_python_ffi: start_wasm error: ", e);
    }
    return parts;
}


async function rust_sqlite_ffi__create(name, msg_code) {
    let parts = make_msg_pair(name);
    try {
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_sqlite_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_sqlite_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);
        module.Xdom_Logger_Util.__set_loggers__wasm(name);
        link_msg_port(name, parts.port, module, async function () {
            var rust_handle = await module.Rust_Sqlite_Ffi.rust_sqlite_ffi__create(name, msg_code, null);
        });
    } catch (e) {
        console.log("rust_sqlite_ffi: start_wasm error: ", e);
    }
    return parts;
}


async function rust_sheet_ffi__create(name, msg_code) {
    let parts = make_msg_pair(name);
    try {
        const url_client_w_js = URL.createObjectURL(msg_code.b_w_sheet_js);
        const url_client_w_bg_wasm = URL.createObjectURL(msg_code.b_w_sheet_bg_wasm);
        var module = await import(url_client_w_js);
        await module.default(url_client_w_bg_wasm);
        module.Xdom_Logger_Util.__set_loggers__wasm(name);
        link_msg_port(name, parts.port, module, async function () {
            var rust_handle = await module.Rust_Sheet_Ffi.rust_sheet_ffi__create(name, msg_code, null);
        });
    } catch (e) {
        console.log("rust_sheet_ffi: start_wasm error: ", e);
    }
    return parts;
}







