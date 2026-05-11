pub struct Framebuffer {
    addr: *mut u32,
    width: usize,
    height: usize,
    pitch: usize,
}

impl Framebuffer {
    pub fn new(addr: u64, width: usize, height: usize, pitch: usize) -> Self {
        Framebuffer {
            addr: addr as *mut u32,
            width,
            height,
            pitch,
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let offset = y * (self.pitch / 4) + x;
            unsafe {
                self.addr.add(offset).write_volatile(color);
            }
        }
    }

    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }
}
