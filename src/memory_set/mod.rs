pub struct MemoryAttr(u32);

// 了解riscv32页表项/页目录项 的结构，根据其结构设置相关属性即可
impl MemoryAttr {
    // 由于我们创建的页表需要是有效（valid）的，所以 new 函数中使用 1 进行初始化
    pub fn new() -> MemoryAttr {
        MemoryAttr(1)
    }

    pub fn set_readonly(mut self) -> MemoryAttr {
        self.0 = self.0 | 0b10;
        self
    }

    pub fn set_execute(mut self) -> MemoryAttr {
        self.0 = self.0 | 0b1000;
        self
    }

    pub fn set_WR(mut self) -> MemoryAttr {
        self.0 = self.0 | 0b10 | 0b100;
        self
    }
}