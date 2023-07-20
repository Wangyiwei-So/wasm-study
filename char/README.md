本示例知识点：
1. rust对象导出为js对象, 并在js中使用
2. js给rust传char的值

{
    "scripts": {
      "build": "webpack",
      "serve": "webpack serve"
    },
    "devDependencies": {
      "@wasm-tool/wasm-pack-plugin": "1.5.0",
      "html-webpack-plugin": "^5.3.2",
      "text-encoding": "^0.7.0",
      "webpack": "^5.49.0",
      "webpack-cli": "^4.7.2",
      "webpack-dev-server": "^4.15.1"
    }
}
package.json文件是一个Node.js项目的文件
其中包含了项目的依赖和脚本
写了package.json文件就可以用npm来使用它

比如下载依赖
npm install

我在package.json文件里定义了两个命令，分别是build和serve。那么我就可以npm run build 和npm run serve