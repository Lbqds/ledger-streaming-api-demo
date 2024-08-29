#![no_std]
#![no_main]

#[cfg(any(target_os = "stax", target_os = "flex"))]
use handler::{Handler, Ins, APP_ICON};
use ledger_device_sdk::io;
#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{init_comm, NbglHomeAndSettings};

mod error_code;
#[cfg(any(target_os = "stax", target_os = "flex"))]
mod handler;

ledger_device_sdk::set_panic!(ledger_device_sdk::exiting_panic);

// This function is the app entry point
#[no_mangle]
extern "C" fn sample_main() {
    let mut comm = io::Comm::new();

    #[cfg(not(any(target_os = "stax", target_os = "flex")))]
    {
        panic!();
    }

    #[cfg(any(target_os = "stax", target_os = "flex"))]
    {
        init_comm(&mut comm);

        let mut handler = Handler::new();
        loop {
            let event = NbglHomeAndSettings::new()
                .glyph(&APP_ICON)
                .infos(
                    "StreamingApiDemo",
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_AUTHORS"),
                )
                .show::<Ins>();
            if let io::Event::Command(ins) = event {
                match handler.handle_apdu(&mut comm, ins) {
                    Ok(_) => comm.reply_ok(),
                    Err(sw) => comm.reply(sw),
                }
            }
        }
    }
}
