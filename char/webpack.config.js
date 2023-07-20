// path模块用于处理文件路径
const path = require('path'); 
// 用于自动生成html文件并将打包后的js文件插入到html中
// webpack本身用于js文件的处理和打包
// 想要生成html文件就需要这个插件
const HtmlWebpackPlugin = require('html-webpack-plugin');
// webpack核心模块 必须要的
const webpack = require('webpack');
// 用于编译和打包rust编写的wasm模块
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

// module.exports将一个对象暴露为模块， 属性就是webpack的配置选项
module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js', //打包后的文件是./dist/index.js
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "index.html",
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ], 
    mode: 'development',
    experiments: { //启用了异步wasm加载功能
        asyncWebAssembly: true
    }
};
