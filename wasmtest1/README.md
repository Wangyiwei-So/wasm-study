该示例演示了最基本的wasmbindgen和web-sys的使用

运行：
npm install html-webpack-plugin --save-dev
npm run serve

编译后会产生一个pkg文件夹，其中有一个.wasm文件和
一个.js文件， .wasm文件就是编译出的wasm模块，
而.js文件用于加载该wasm模块到内存并生成WebAssembly.Module对象。

所以index.js中只要import "./pkg"即可把wasm模块加载


wasm-bindgen用于提供方便的rust和javascript交互的功能
web-sys是提供浏览器的Web API