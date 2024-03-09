mod front_of_house {  // 同根级可以互相调用
    pub mod hosting { // pub
        pub fn add_to_waitlist() {} // pub
    }
}

pub fn eat_at_restaurant() {
    // eat_at_restaurant函数和front_of_house模块都在同一个crateRoot里(lib.rs)
    crate::front_of_house::hosting::add_to_waitlist(); // 绝对路径 从crate开始  (私有的)
    front_of_house::hosting::add_to_waitlist(); // 相对路径 (同一子目录)
}
