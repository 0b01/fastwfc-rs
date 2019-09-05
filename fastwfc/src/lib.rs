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

fn destroy_arr_2d(c: *mut fastwfc_sys::ArrayColor2D) {
    // if there is a Vec reference originally allocated from Rust
    // Drop it by converting it back to a Box
    if ::std::ptr::null() != unsafe{(*c).ref_} {
        let ptr = unsafe { fastwfc_sys::array_color_2d_get_ref(c) } as *mut Vec<u8>;
        let obj: Box<Vec<u8>> = unsafe { Box::from_raw(ptr) };
        ::std::mem::drop(obj);
    }
    // delete ArrayColor2D that C++ allocated
    unsafe { fastwfc_sys::destroy_array_color_2d(c); }
}

/// Generat a new image with the overlapping WFC algorithm
#[derive(Clone, Copy)]
pub struct Overlapping {
    /// Toric input, defaults to false
    pub periodic_input: bool,
    /// Toric output, defaults to false
    pub periodic_output: bool,
    /// The height of the output in pixels
    pub out_height: u64,
    /// The width of the output in pixels
    pub out_width: u64,
    /// Number of symmetries from 0 to 8
    /// If the pattern already exist, increase its number of appearance.
    pub symmetry: u8,
    /// Output image contains ground
    /// > The lowest middle pattern is used as a floor (and ceiling when the input is
    /// > toric) and is placed at the lowest possible pattern position in the output
    /// > image, on all its width. The pattern cannot be used at any other place in
    /// > the output image.
    ///
    /// Defaults to false
    pub ground: bool,
    /// Width and height in pixel of the patterns
    pub pattern_size: u64,
}

impl Overlapping {
    /// Create a new Overlapping WFC generator
    pub fn new( out_height: u64, out_width: u64, pattern_size: u64) -> Self {
        Self {
            periodic_input: false,
            periodic_output: false,
            out_height,
            out_width,
            symmetry: 8,
            ground: false,
            pattern_size,
        }
    }

    /// Set number of symmetries.
    pub fn symmetry(&mut self, value: u8) -> &mut Self {
        self.symmetry = value;
        self
    }

    /// Set whether the generated image contains ground
    pub fn ground(&mut self, value: bool) -> &mut Self {
        self.ground = value;
        self
    }

    /// Set whether input is toric
    pub fn periodic_input(&mut self, value: bool) -> &mut Self {
        self.periodic_input = value;
        self
    }

    /// Set whether output is toric
    pub fn periodic_output(&mut self, value: bool) -> &mut Self {
        self.periodic_output = value;
        self
    }

    fn as_ffi_opts(&self) -> fastwfc_sys::OverlappingWFCOptions {
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

    /// Generate WFC based on input image
    pub fn generate(&self, input: image::RgbaImage, tries: u32) -> Option<image::RgbaImage> {
        let array2d = to_array_color_2d(input);
        let ret = unsafe { run_overlapping(array2d, self.as_ffi_opts(), tries) };
        destroy_arr_2d(array2d);
        let result = from_array_color_2d(ret);
        destroy_arr_2d(ret);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_run_overlapping() {
        let runner = Overlapping::new(100, 100, 2);
        let input = image::open("../fastwfc-sys/fast-wfc/example/samples/Chess.png")
            .unwrap()
            .to_rgba();
        let output = runner.generate(input, 100);
        assert!(output.is_some());
    }
}
