use hidapi;


fn connect_to_mouse() -> hidapi::HidResult<()>{

    let mut data: [u8; 154] = [ 0 ; 154];

    data[0x00] = 0x4; // Report ID

    data[0x08] = 0x3c; // Constant data

    data[0x4A] = 01; // DPI - 01 - (00 - 0a)
    data[0x4B] = 02; // DPI - 02 - (00 - 0a)
    data[0x4C] = 04; // DPI - 03 - (00 - 0a)
    data[0x4D] = 08; // DPI - 04 - (00 - 0a) 

    data[0x04E..0x05a].copy_from_slice(&[0x80;12]);  // Had a bunch of 0x80


    data[0x5d] = 0x44; // 0x28 for steady mode or 0x22 for breathe mode
    data[0x60] = 0x30; // Brightness level at steady mode or speed at breathe mode.
    data[0x5f] = 0x01;
    data[0x5e] = 0x0a;


    data[0x64] = 0xFF;// RGB values , 1st profile
    data[0x65] = 0x00;
    data[0x66] = 0x00;

    data[0x67] = 0x00;// RGB values , 2nd profile 
    data[0x68] = 0x00;
    data[0x69] = 0xFF;

    data[0x6a] = 0xFF;// RGB values , 3rd profile
    data[0x6b] = 0x00;
    data[0x6c] = 0xFF;

    data[0x6d] = 0x00;// RGB values , 4th profile
    data[0x6e] = 0x00;
    data[0x6f] = 0xFF;
   
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
    // Not sure of the significance

    data[0x94] = 0x58; // XY - 801 Signature
    data[0x95] = 0x59;
    data[0x96] = 0x2D;
    data[0x97] = 0x38;
    data[0x98] = 0x30;
    data[0x99] = 0x01;

    // Data being sent:
    println!("Data being sent!");
    let mut i:u8 = 0;
    data.iter().for_each(|e|{
        if i < 15 {
            print!("{:?}  ",e);
            i+=1;
        }else {
            print!("{:?}  ",e);
            print!("\r\n");
            i=0;
        }
    });

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
