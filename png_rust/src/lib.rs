enum PngInterlace {
    ADAM7,
}

pub struct Png {
    pass: u8,        /* current interlace pass (0 - 6) */
    compression: u8, /* file compression type (always 0) */
    interlaced: Option<PngInterlace>,
    filter: u8,      /* file filter type (always 0) */
}

impl Drop for Png {
    fn drop(&mut self) {
        // Do something "interesting" in the destructor so we know when it gets
        // called
        println!("Destroying rust obj");
    }
}

#[no_mangle]
pub extern fn png_rust_new() -> *mut Png
{
    let obj = Box::new(Png {
        pass : 0,
        compression : 0,
        interlaced: None,
        filter : 0
    });
    Box::into_raw(obj)
}

#[no_mangle]
pub extern fn png_rust_free(state: *mut Png)
{
}

#[no_mangle]
pub extern fn png_rust_pass_is_valid(this: *const Png) -> bool
{
    unsafe {
        (*this).pass <= 6
    }
}

#[no_mangle]
pub extern fn png_rust_get_pass(this: *const Png) -> u8
{
    unsafe {
        (*this).pass
    }
}

#[no_mangle]
pub extern fn png_rust_incr_pass(this: *mut Png)
{
    unsafe {
        (*this).pass += 1;
    }
}

#[no_mangle]
pub extern fn png_rust_decr_pass(this: *mut Png)
{
    unsafe {
        (*this).pass -= 1;
    }
}

#[no_mangle]
pub extern fn png_rust_get_interlace(this: *const Png) -> i32
{
    let interlace;
    unsafe {
        interlace = &(*this).interlaced;
    }

    match interlace {
        Some(interlace) => {
            match interlace {
                PngInterlace::ADAM7 => 1,
            }
        },
        None => 0,
    }
}

#[no_mangle]
pub extern fn png_rust_set_interlace(this: *mut Png, value: i32)
{
    let interlace = match value {
        0 => None,
        1 => Some(PngInterlace::ADAM7),
        _ => {
            println!("Invalid interlace value ({}), use none instead", value);
            None
        }
    };

    unsafe {
        (*this).interlaced = interlace;
    }
}
