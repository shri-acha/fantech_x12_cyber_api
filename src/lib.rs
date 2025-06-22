mod hid;
mod structures;
mod packet;
use std::rc::Rc;
use std::{io, usize};
use hid::connect_to_mouse;
// Just an identifier for Addresses

fn safe_assign_to_packet(data: Rc<[u8; 154]>) -> io::Result<()> {
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
