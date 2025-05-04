use hidapi;
use rusb::Device;


fn connect_to_mouse() -> hidapi::HidResult<()>{

    let mut data: [u8; 155] = [ 0 ; 155];
    data[0x00] = 0x4;
    // bRequest = 9, no clue what this is 
    // data[0x01] = 0x4;// ReportID = 04, this is a required field for two direction packet transfet
    // data[0x02] = 0x3;// ReportType = 03, unsure
    // data[0x03] = 0x1;//  wIndex = 1, data index
    // data[0x06] = 0x9a;// wLength = 154 , length of data segment

    data[0x65] = 0x22; // Something i believe
    data[0x68] = 0x12; // Brightness level
    data[0x6C] = 0xFF;// RGB values , RGB values +1 on the values because 0th value is not held by
                      // requestID
    data[0x6D] = 0xFF;
    data[0x6E] = 0xFF;

    let mut device_list = vec![];
    if let Ok(hid_ctx) = hidapi::HidApi::new(){
        for device in hid_ctx.device_list() {
            device_list.push(device); 
            println!("{:?} {:?}",device.interface_number(),device);
            if device.vendor_id() == 9610 && device.product_id() == 23 && device.interface_number() == 1 {
                if let Ok(m_handl) = hid_ctx.open(0x258a,0x17){
                    if let Err(e) = m_handl.send_feature_report(&data){
                        panic!("{}",e);
                    }
                    else{
                        println!("Worked!");
                        break;
                    }
                }else {
                    println!("Didn't Work!");
                }
                break;
            }
        }
        
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_connection() {
        let _ = connect_to_mouse();
    }
}
