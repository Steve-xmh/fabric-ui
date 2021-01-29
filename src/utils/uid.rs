//! 独一标识符：确保每次新建控件时都会得到不一样的标识符

static mut COUNTER: usize = 0;

pub fn gen_uid() -> usize {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
