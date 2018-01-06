
(() => {
    let wasm = new WASM();
    wasm.fetch('target/wasm32-unknown-unknown/release/wasm_test.wasm').then(main);

    function main(){
        console.log(wasm.sumint(-3, 5));
        console.log(wasm.sumfloat(-1.5, 3.25));
        console.log(wasm.sumsize(-100000, 200000));
        console.log(restoreString());
    }

    function restoreString(){
        let offset = wasm.getstring();
        let length = wasm.getstringlength();
        let buffer = new Uint8Array(wasm.memory.buffer, offset, length);
        let decoder = new TextDecoder('UTF-8');
        let str = decoder.decode(buffer);
        return str;

        // let str = '';
        // for(let i = 0; i < buffer.length; ++i){
        //     str += String.fromCharCode(buffer[i]);
        // }
        // return str;
    }
})();

