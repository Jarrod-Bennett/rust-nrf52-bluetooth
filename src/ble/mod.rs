// use hal::pac::{self, FICR, RADIO};
// use nrf52840_hal as hal;
//
// use rubble_nrf5x::radio::{BleRadio, PacketBuffer};
//
// struct BluetoothBuffer {
//     tx_buf: PacketBuffer,
//     rx_buf: PacketBuffer,
// }
//
// pub struct BluetoothConnection {
//     radio: BleRadio,
//     buffers: BluetoothBuffer,
// }
//
// // impl BluetoothConnection {
// //     pub fn new(radio: RADIO, ficr: &FICR) -> BluetoothConnection {
// //
// //         let mut buffer = BluetoothBuffer {
// //             tx_buf: [0; 39],
// //             rx_buf: [0; 39],
// //         };
// //
// //         // let mut tx_buf: PacketBuffer = [0; 39];
// //         // let mut rx_buf: PacketBuffer = [0; 39];
// //
// //         BluetoothConnection {
// //             radio: BleRadio::new(radio, ficr, &mut buffer.tx_buf, &mut buffer.rx_buf),
// //             buffers: buffer,
// //         }
// //     }
// // }
