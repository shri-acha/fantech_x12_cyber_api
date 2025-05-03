use std::str::Bytes;

use hidapi;

fn connect_to_mouse(){

    let mut data: [u8; 155] = [ 0 ; 155];
    data[0x00] = 0x1c; // Report ID
    data[0x88] = 0xFD; // RGB values
    data[0x89] = 0xFE;
    data[0x8A] = 0xFF;

    if let Ok(hid_ctx) = hidapi::HidApi::new(){
        let m_handl = hid_ctx.open(0x258a,0x17).unwrap();
        let _ = m_handl.read(&mut data); 
        // let result = m_handl.send_feature_report(&data).unwrap();
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_connection() {
        let result = connect_to_mouse();
    }
}
