const wasm = await import("gravy-galaxy");

//  TODO: uhh figure out correct way to do this
// @ts-ignore
await wasm.default.run();

document.getElementsByTagName("body")[0].innerHTML +=
    '<p style= "z-index:-1; font-size: 1.6rem; position: absolute;">This should be covered by a canvas</p>';

export default {};
