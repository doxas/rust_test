
/**
 * WebAssembly utility
 * @class
 */
class WASM {
    /**
     * @constructor
     */
    constructor(){
        this.instance = null;
    }
    /**
     * fetch wasm
     * @param {string} target - wasm file path
     * @return {promise} wasm instance to args
     */
    fetch(target){
        return WASM.fetchAndInstantiate(target)
            .then((instance) => {
                this.instance = instance;
                if(this.instance != null){
                    for(let i in this.instance.exports){
                        this[i] = this.instance.exports[i];
                    }
                }
                return instance;
            });
    }
    /**
     * return wasm function
     * @return {object} wasm function
     */
    getWasmFunctions(){
        return this.instance ? this.instance.exports : null;
    }
    /**
     * fetch and instantiate for wasm file
     * @param {string} target - wasm file path
     * @return {promise} wasm instance to args
     */
    static fetchAndInstantiate(target){
        return fetch(target)
            .then((response) => {
                return response.arrayBuffer();
            })
            .then((bytes) => {
                return WebAssembly.instantiate(bytes);
            })
            .then((result) => {
                return result.instance;
            });
    }
}

