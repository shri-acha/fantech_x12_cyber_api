use std::cell::RefCell;
use std::collections::hash_map::HashMap;

pub fn change_rgb_value(
    data: Rc<RefCell<[u8; 154]>>,
    rgb: (u8, u8, u8),
    section_no: u8,
) -> io::Result<()> {
    let mut data = *data.borrow();

    if section_no > 4 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Error"));
    }

    if section_no == 1 {
        data[0x64] = rgb.0; // RGB values , 1st profile
        data[0x65] = rgb.1;
        data[0x66] = rgb.2;
    } else if section_no == 2 {
        data[0x67] = rgb.0; // RGB values , 2nd profile
        data[0x68] = rgb.1;
        data[0x69] = rgb.2;
    } else if section_no == 3 {
        data[0x6a] = rgb.0; // RGB values , 3rd profile
        data[0x6b] = rgb.1;
        data[0x6c] = rgb.2;
    } else if section_no == 4 {
        data[0x6d] = rgb.0; // RGB values , 4th profile
        data[0x6e] = rgb.1;
        data[0x6f] = rgb.2;
    }
    Ok(())
}
