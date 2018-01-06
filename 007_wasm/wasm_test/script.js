
(() => {
    let wasm = new WASM();
    wasm.fetch('target/wasm32-unknown-unknown/release/wasm_test.wasm').then(main);

    function main(){
        console.log(wasm.sumint(-3, 5));
        console.log(wasm.sumfloat(-1.5, 3.25));
        console.log(wasm.sumsize(-100000, 200000));
    }
})();


