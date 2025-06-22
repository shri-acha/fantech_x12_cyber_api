use hidapi;
use crate::packet::prepare_packet;
pub fn connect_to_mouse() -> hidapi::HidResult<()> {
    let mut data = [0; 154];

    data = prepare_packet(data)?;
    println!("data: {:?}", data);
    // Data being sent:
    // let mut i:u8 = 0;
    // data.iter().for_each(|e|{
    //     if i < 15 {
    //         print!("{:?}  ",e);
    //         i+=1;
    //     }else {
    //         print!("{:?}  ",e);
    //         print!("\r\n");
    //         i=0;
    //     } });

    let mut device_list = vec![];
    if let Ok(hid_ctx) = hidapi::HidApi::new() {
        for device in hid_ctx.device_list() {
            if device.vendor_id() == 9610 && device.product_id() == 23 {
                println!(
                    "{:?} {:?} {:?} {:?}",
                    device.interface_number(),
                    device,
                    device.product_id(),
                    device.bus_type()
                );
                device_list.push(device);
            }
        }
        let m_handl = device_list.pop().unwrap().open_device(&hid_ctx);
        if let Ok(value) = m_handl {
            if let Err(e) = value.send_feature_report(&data) {
                panic!("{}", e);
            } else {
                println!("Data Sent to Mouse!");
            }
        } else {
            println!("Error finding handle to device!");
        }
    }
    Ok(())
}
