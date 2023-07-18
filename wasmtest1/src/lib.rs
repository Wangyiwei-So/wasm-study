use wasm_bindgen::prelude::*;

//#[wasm_bindgen(start)] 标记了一个函数是WASM的入口函数
// 在模块实例化后会自动调用该函数
// 一个wasm模块只能有一个入口函数
#[wasm_bindgen(start)]
fn run(){
    log("hello rust");
    using_web_sys();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

//使用web_sys库来实现同样的console功能。会更高级，更方便
fn using_web_sys(){
    use web_sys::console; 
    console::log_1(&"hello web-sys".into());
}