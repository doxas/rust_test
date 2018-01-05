
(() => {
    let wasmFunctions = null;
    fetch('target/wasm32-unknown-unknown/release/wasm_test.wasm')
        .then((response) => {
            return response.arrayBuffer();
        })
        .then((bytes) => {
            return WebAssembly.instantiate(bytes);
        })
        .then((result) => {
            wasmFunctions = result.instance.exports;
            console.log(wasmFunctions);
            console.log(wasmFunctions.sum(5, 10));
        });
})();

