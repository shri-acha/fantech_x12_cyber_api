use crate::structures::AddrEnum;
use crate::io;
pub fn get_address(addr_name: AddrEnum) -> Option<i8> {
    match addr_name {
        AddrEnum::ReportID => {
            return Some(0x00);
        }
        AddrEnum::Dpi1 => {
            return Some(0x4A);
        }
        AddrEnum::Dpi2 => {
            return Some(0x4B);
        }
        AddrEnum::Dpi3 => {
            return Some(0x4C);
        }
        AddrEnum::Dpi4 => {
            return Some(0x4D);
        }
        AddrEnum::RGB1 => {
            return Some(0x64);
        }
        AddrEnum::RGB2 => {
            return Some(0x67);
        }
        AddrEnum::RGB3 => {
            return Some(0x6a);
        }
        AddrEnum::RGB4 => {
            return Some(0x6D);
        }
        AddrEnum::Mode => {
            return Some(0x5D); // 0x28 for steady mode or 0x22 for breathe mode
        }
        AddrEnum::Speed => {
            return Some(0x60);
        }
        AddrEnum::StdBri => {
            return None;
        }
        AddrEnum::BrthFr => {
            return None;
        }
        AddrEnum::BlModeFr => {
            return None;
        }
        AddrEnum::BlModeTi => {
            return None;
        }
        AddrEnum::Brightness => {
            return Some(0x60);
        }
        AddrEnum::BlModeLightingStdy => return None,
        AddrEnum::BiModeLightingNeon => {
            return None;
        }
        AddrEnum::BlModeLightingRespiration => {
            return None;
        }
        _ => {
            return None;
        }
    }
}
pub fn prepare_packet(mut data: [u8; 154]) -> io::Result<[u8; 154]> {
    //Default Configuration
    data[get_address(AddrEnum::ReportID).expect("[ERROR]") as usize] = 0x4; // Report ID
    data[0x08] = 0x3c; // Constant data

    //Variables
    data[get_address(AddrEnum::Dpi1).expect("[ERROR]") as usize] = 01; // DPI - 01 - (00 - 0a)
    data[get_address(AddrEnum::Dpi2).expect("[ERROR]") as usize] = 02; // DPI - 01 - (00 - 0a)
    data[get_address(AddrEnum::Dpi3).expect("[ERROR]") as usize] = 04; // DPI - 01 - (00 - 0a)
    data[get_address(AddrEnum::Dpi4).expect("[ERROR]") as usize] = 08; // DPI - 01 - (00 - 0a)

    data[0x47] = 10; // Profile Index ?
    data[0x48] = 24; //

    data[0x04E..0x05A].copy_from_slice(&[0x80; 12]); // Had a bunch of 0x80

    //Variables
    data[get_address(AddrEnum::Mode).expect("[ERROR]") as usize] = 0x44; // 0x28 for steady mode or 0x22 for breathe mode
    data[get_address(AddrEnum::Brightness).expect("[ERROR]") as usize] = 0x1f; // Brightness level at steady mode or speed at breathe mode or speed
    data[get_address(AddrEnum::Speed).expect("[ERROR]") as usize] = 0x1f; // Brightness level at steady mode or speed at breathe mode or speed
                                                                          // at neon? idk, i forgot
    data[0x5e] = 0x0a;
    data[0x5f] = 0x02; // Constants les assume

    data[get_address(AddrEnum::RGB1).expect("[ERROR]") as usize] = 0xFF; // RGB values , 1st profile
    data[get_address(AddrEnum::RGB1).expect("[ERROR]") as usize + 1] = 0x00;
    data[get_address(AddrEnum::RGB1).expect("[ERROR]") as usize + 2] = 0xFF;

    data[get_address(AddrEnum::RGB2).expect("[ERROR]") as usize] = 0x00; // RGB values , 2nd profile
    data[get_address(AddrEnum::RGB2).expect("[ERROR]") as usize + 1] = 0x00;
    data[get_address(AddrEnum::RGB2).expect("[ERROR]") as usize + 2] = 0xFF;

    data[get_address(AddrEnum::RGB3).expect("[ERROR]") as usize] = 0xFF; // RGB values , 3rd profile
    data[get_address(AddrEnum::RGB3).expect("[ERROR]") as usize + 1] = 0x00; // RGB values , 3rd profile
    data[get_address(AddrEnum::RGB3).expect("[ERROR]") as usize + 2] = 0xFF; // RGB values , 3rd profile

    data[get_address(AddrEnum::RGB4).expect("[ERROR]") as usize] = 0x00; // RGB values , 4th profile
    data[get_address(AddrEnum::RGB4).expect("[ERROR]") as usize + 1] = 0x00; // RGB values , 4th profile
    data[get_address(AddrEnum::RGB4).expect("[ERROR]") as usize + 2] = 0x00; // RGB values , 4th profile

    //Constant Data
    //
    data[0x70] = 0xFF;
    data[0x71] = 0xFF;
    data[0x74] = 0xFF;
    data[0x75] = 0xFF;
    data[0x76] = 0xFF;
    data[0x77] = 0xFF;
    data[0x78] = 0xFF;
    data[0x7c] = 0xFF;
    data[0x80] = 0xFF;
    data[0x84] = 0xFF;
    data[0x85] = 0xFF;
    data[0x86] = 0xFF;
    data[0x87] = 0xFF;
    data[0x88] = 0xFF;
    data[0x8a] = 0xFF;
    data[0x8c] = 0xFF;
    data[0x8d] = 0xFF;
    data[0x8e] = 0xFF;
    data[0x8f] = 0xFF;
    data[0x90] = 0xFF;
    // Probably bit masks or stuffed bits

    data[0x94] = 0x58; // XY - 801 Signature
    data[0x95] = 0x59;
    data[0x96] = 0x2D;
    data[0x97] = 0x38;
    data[0x98] = 0x30;
    data[0x99] = 0x01;

    Ok(data)
}
