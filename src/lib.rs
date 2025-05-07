use hidapi;
use rusb::Device;


fn connect_to_mouse() -> hidapi::HidResult<()>{

    let mut data: [u8; 154] = [ 0 ; 154];
    data[0x00] = 0x4;

    // bRequest = 9, no clue what this is 
    // data[0x01] = 0x4;// ReportID = 04, this is a required field for two direction packet transfet
    // data[0x02] = 0x3;// ReportType = 03, unsure
    // data[0x03] = 0x1;//  wIndex = 1, data index
    // data[0x06] = 0x9a;// wLength = 154 , length of data segment

    data[0x08] = 0x30; // Constant data

    data[0x64] = 0x22; // 0x28 for steady mode. 
    data[0x67] = 0x12; // Brightness level at Stale mode.


    data[0x4A] = 01; // DPI (00 - 0a)
    data[0x4B] = 02;
    data[0x4C] = 04;

    data[0x6A] = 0xFF;// RGB values , RGB values +1 on the values because -1th value is not held by requestID
    data[0x6B] = 0xFF;
    data[0x6C] = 0xFF;

    data[0x94] = 0x58; // XY - 801 Signature
    data[0x95] = 0x59;
    data[0x96] = 0x2D;
    data[0x97] = 0x38;
    data[0x98] = 0x30;
    data[0x99] = 0x01;


    let mut device_list = vec![];
    if let Ok(hid_ctx) = hidapi::HidApi::new(){
        for device in hid_ctx.device_list() {
            if device.vendor_id() == 9610 && device.product_id() == 23 {
                println!("{:?} {:?} {:?} {:?}",device.interface_number(),device,device.product_id(),device.bus_type());
                device_list.push(device); 
            }
        }
        let m_handl = device_list.pop().unwrap().open_device(&hid_ctx);
        if let Ok(value) = m_handl{
            if let Err(e) = value.send_feature_report(&data){
                panic!("{}",e);
            }
            else{
                println!("Data Sent to Mouse!");
            }
        }else {
            println!("Error finding handle to device!");
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
