#[cfg(windows)] extern crate winapi;

#[cfg(windows)]
pub fn set_cursor_pos(x: i32, y: i32) {
    use winapi::um::winuser::SetCursorPos;
    unsafe {
        SetCursorPos(x,y);
    }
}