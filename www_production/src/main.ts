import * as wasm from "gravy-galaxy";
console.log(wasm);
//  TODO: uhh fix this
// @ts-ignore
wasm.init();

document.getElementsByTagName("body")[0].innerHTML +=
    '<p style= "z-index:-1; font-size: 1.6rem; position: absolute;">This should be covered by a canvas</p>';

export default {};
