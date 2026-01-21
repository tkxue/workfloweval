
// let func_name = ... // from rust land




// console.log("lsp: (worker) start");
onmessage = (event) => {

    // console.log("lsp: (worker)", event);

    if (event.data.length === 1) {
        let data = event.data[0];
        let js_code = data[0];
        let wasm_code = data[1];


        // console.log("lsp: (worker) got js_code, wasm_code", js_code, wasm_code);


        try {
            let enc = new TextDecoder("utf-8");
            let s = enc.decode(new Uint8Array(js_code));
            const encodedJs = encodeURIComponent(s);
            const dataUri = 'data:text/javascript;charset=utf-8,' + encodedJs;
            import(dataUri).then((module) => {{
                module.default(wasm_code).then(() => {{
                    module.Rust_Util.start_process(js_code, wasm_code, func_name );
                }});
            }});

        } catch(e) {
            console.log("lsp: (worker) error:", e)
        }



        onmessage = null;

    }
};

