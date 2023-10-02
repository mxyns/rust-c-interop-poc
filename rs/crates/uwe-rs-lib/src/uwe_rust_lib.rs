use std::{ptr, slice};
use std::io::Cursor;
use rand;
use libc;
use byteorder::{NetworkEndian, ReadBytesExt};

/// Rust function with constant return value that can be called from C
#[no_mangle]
pub extern "C" fn is_uwe_the_goat() -> bool {
    true
}

/// Rust function with variable return value that can be called from C
#[no_mangle]
pub extern "C" fn get_random_value() -> f32 {
    rand::random()
}

/// Rust function with parameter called from C
#[no_mangle]
pub extern "C" fn print_my_number(my_number: u8) {
    println!("My C unsigned byte is {}", my_number)
}

/// Some Rust structure holding simple values that can be exported to C
#[repr(C)]
pub struct TransparentHeader {
    pkt_type: u16,
    pkt_len: u32
}

impl TransparentHeader {
    const BASE_LEN: usize = 2 /* pkt_type */ + 4 /* pkt_len */;
}

/// Complex function which takes values from C and returns a Rust struct allocated by Rust
/// Also uses 3rd party libraries
#[no_mangle]
pub extern "C" fn read_transparent_buffer(buffer: *const libc::c_char, buffer_length: libc::size_t) -> *mut TransparentHeader {

    /* Pointers can be null */
    if buffer.is_null() {
        print!("read_buffer received a null pointer shame on you C user");
        return ptr::null_mut()
    }

    let slice: &[u8] = unsafe {
        slice::from_raw_parts(buffer as *const u8, buffer_length)
    };

    if slice.len() < TransparentHeader::BASE_LEN {
        print!("read_buffer received a buffer too small for a TransparentStruct shame on you C user");
        return ptr::null_mut()
    }

    let mut cursor = Cursor::new(slice);
    let read_type = cursor.read_u16::<NetworkEndian>().unwrap();
    let read_len = cursor.read_u32::<NetworkEndian>().unwrap();

    /* Allocate, make a pointer, and tell Rust to not drop the value when it goes
     * out of scope
     */
    Box::into_raw(Box::new(TransparentHeader {
        pkt_type: read_type,
        pkt_len: read_len,
    }))
}

/// Make Rust free a pointer to a TransparentHeader
/// (Really C does check shit so it could be a pointer to anything Rust has allocated)
/// (I'm not sure what happens if it's a pointer to something C allocated though)
#[no_mangle]
pub extern "C" fn free_transparent_header(ptr: *mut TransparentHeader) -> bool {

    if ptr.is_null() {
        false
    } else {
        unsafe {
            let _ = Box::from_raw(ptr);
        };

        true
    }


}