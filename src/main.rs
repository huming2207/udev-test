extern crate udev;

fn main() {
  let mut enumerator = udev::Enumerator::new().unwrap();

  enumerator.match_property("ID_USB_DRIVER", "cdc_acm").unwrap();

  for device in enumerator.scan_devices().unwrap() {
    println!("found device: {:?}", device.sysname());
    for property in device.properties() {
        println!("\tProperty: {:?}; Value {:?}", property.name(), property.value());
    }
    
  }
}
