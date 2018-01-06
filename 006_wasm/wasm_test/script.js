
(() => {
    let wasm = new WASM();
    wasm.fetch('target/wasm32-unknown-unknown/release/wasm_test.wasm').then(main);

    function main(){
        console.log(wasm.sum(3, 5));
    }
})();


