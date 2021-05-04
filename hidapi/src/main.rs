use std::str::from_utf8;

use faster_hex::hex_encode;
use hidapi::HidApi;

trait ReplaceEmpty {
    fn replace_empty(self, default: Self) -> Self
    where
        Self: Sized;
}

impl ReplaceEmpty for &str {
    fn replace_empty(self, default: Self) -> Self
    where
        Self: Sized,
    {
        match self.is_empty() {
            true => default,
            _ => self,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize HIDAPI.  Only one `HidApi` instance can exist at any given time, and as of 1.2.6
    // this is enforced by an internal lock.
    let hidapi = HidApi::new()?;

    // `device_list()` uses cached information, so `refresh_device_list()` is necessary
    // if `hidapi` has not been initialized immediately before this call.
    for device in hidapi.device_list() {
        let manufacturer = device
            .manufacturer_string()
            .unwrap_or("")
            .trim()
            .replace_empty("<manufacturer>");
        let product = device
            .product_string()
            .unwrap_or("")
            .trim()
            .replace_empty("<product>");
        let serial = device
            .serial_number()
            .unwrap_or("")
            .trim()
            .replace_empty("<serial number>");

        println!(
            "Found {} {} {} ({:04x}:{:04x}) at {}",
            manufacturer,
            product,
            serial,
            device.vendor_id(),
            device.product_id(),
            device.path().to_string_lossy()
        );

        // These are only not `None` when the data is not valid UTF-8; that is, when the non-raw
        // alternatives return `None`.  Also note that these return `Option<&[wchar_t]>`, which
        // isn't easily (and lossy) convertible to &str with the standard library.
        dbg!(device.manufacturer_string_raw());
        dbg!(device.product_string_raw());
        dbg!(device.serial_number_raw());

        match device.open_device(&hidapi) {
            Ok(device) => {
                let mut buf = [0; 256];

                // this should be safe with any device that can be opened, but...
                let len = device.read_timeout(&mut buf, 1000)?;
                dbg!(len);

                let mut hbuf = [0; 512];
                hex_encode(&buf[..len], &mut hbuf)?;
                dbg!(from_utf8(&hbuf[..len*2])?);
            }
            Err(err) => {
                dbg!(err);
            }
        };
    }

    Ok(())
}
