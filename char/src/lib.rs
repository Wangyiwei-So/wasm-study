use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

//这里定义一个结构体，通过wasm_bindgen让他可以导出
//导出后可以被js使用
#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32,
}

#[wasm_bindgen]
impl Counter{
    pub fn new(key:char,count: i32)->Counter{
        log(&format!("New Counter with {} {}",key,count));
        Counter { key, count }
    }
    pub fn key(&self) -> char {
        log("Counter.key()");
        self.key
    }

    pub fn count(&self) -> i32 {
        log("Counter.count");
        self.count
    }

    pub fn increment(&mut self) {
        log("Counter.increment");
        self.count += 1;
    }

    pub fn update_key(&mut self, key: char) {
        self.key = key;
    }
}