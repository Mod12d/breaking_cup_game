use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
}

#[wasm_bindgen]
impl Block {
    pub fn new() -> Block {
        Block {
            x: 0,
            y: 0,
            visible: true,
        }
    }

    pub fn update_position(&mut self, new_x: i32, new_y: i32) {
        self.x = new_x;
        self.y = new_y;
    }

    pub fn destroy(&mut self) {
        self.visible = false;
    }
}