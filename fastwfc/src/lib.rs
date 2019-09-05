extern crate fastwfc_sys;
extern crate image;

use fastwfc_sys::run_overlapping;

fn to_array_color_2d(input: image::RgbaImage) -> *mut fastwfc_sys::ArrayColor2D {
    let (w, h) = input.dimensions();
    let obj = Box::new(input.into_vec());
    let arr: *const u8 = obj.as_ptr(); // TODO: memory leak
    let ptr = Box::into_raw(obj);
    unsafe {
        let array2d = fastwfc_sys::new_array_color_2d();
        fastwfc_sys::array_color_2d_set_height(array2d, h as i32);
        fastwfc_sys::array_color_2d_set_width(array2d, w as i32);
        fastwfc_sys::array_color_2d_set_data(
            array2d,
            arr as *mut fastwfc_sys::Color,
            ptr as *mut ::std::os::raw::c_void,
        );
        return array2d;
    }
}

fn from_array_color_2d(c: *mut fastwfc_sys::ArrayColor2D) -> Option<image::RgbaImage> {
    unsafe {
        if !(*c).init {
            return None;
        }
        let w = fastwfc_sys::array_color_2d_get_width(c);
        let h = fastwfc_sys::array_color_2d_get_height(c);
        let data = fastwfc_sys::array_color_2d_get_data(c) as *mut u8;
        let buf = ::std::slice::from_raw_parts(data, (w * h * 4) as usize).to_vec();
        let img = image::RgbaImage::from_raw(w as u32, h as u32, buf);
        img
    }
}

pub struct Overlapping {
    pub periodic_input: bool,
    pub periodic_output: bool,
    pub out_height: u64,
    pub out_width: u64,
    pub symmetry: u64,
    pub ground: bool,
    pub pattern_size: u64,
}

impl Overlapping {
    pub fn new(
        periodic_input: bool,
        periodic_output: bool,
        out_height: u64,
        out_width: u64,
        symmetry: u64,
        ground: bool,
        pattern_size: u64,
    ) -> Self {
        Self {
            periodic_input,
            periodic_output,
            out_height,
            out_width,
            symmetry,
            ground,
            pattern_size,
        }
    }

    fn get_options(&self) -> fastwfc_sys::OverlappingWFCOptions {
        fastwfc_sys::OverlappingWFCOptions {
            periodic_input: self.periodic_input,
            periodic_output: self.periodic_output,
            out_height: self.out_height as u32,
            out_width: self.out_width as u32,
            symmetry: self.symmetry as u32,
            ground: self.ground,
            pattern_size: self.pattern_size as u32,
        }
    }

    pub fn run(&self, input: image::RgbaImage, tries: u32) -> Option<image::RgbaImage> {
        let array2d = to_array_color_2d(input);
        let ret = unsafe { run_overlapping(array2d, self.get_options(), tries) };
        destroy_vec_ref(array2d);
        unsafe {
            fastwfc_sys::destroy_array_color_2d(array2d);
        }
        let result = from_array_color_2d(ret);
        unsafe {
            fastwfc_sys::destroy_array_color_2d(ret);
        }
        result
    }
}

fn destroy_vec_ref(c: *mut fastwfc_sys::ArrayColor2D) {
    let ptr = unsafe { fastwfc_sys::array_color_2d_get_ref(c) } as *mut Vec<u8>;
    let obj: Box<Vec<u8>> = unsafe { Box::from_raw(ptr) };
    ::std::mem::drop(obj);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_run_overlapping() {
        let runner = Overlapping::new(true, true, 100, 100, 1, false, 2);
        let input = image::open("../fastwfc-sys/fast-wfc/example/samples/Chess.png")
            .unwrap()
            .to_rgba();
        let output = runner.run(input, 100);
        println!("{:#?}", output);
        output.unwrap().save("out.png").unwrap();
    }
}
