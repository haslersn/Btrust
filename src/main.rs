#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use blurz::bluetooth_adapter::BluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice;
use blurz::bluetooth_session::BluetoothSession;
use env_logger::Builder;
use env_logger::Env;
use std::error::Error;
use std::time::Duration;

type Result<T = ()> = std::result::Result<T, Box<Error>>;

#[derive(Deserialize)]
struct Config {
    adapter_alias: Option<String>,
}

fn main() -> Result {
    // initialize logger w/ log level "info"
    Builder::from_env(Env::new().default_filter_or("info")).init();

    let config = read_config()?;

    loop {
        if let Err(err) = run(&config) {
            error!("{:?}", err);
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn read_config() -> Result<Config> {
    let conf_str = std::fs::read_to_string("btrust.toml")?;
    toml::from_str(&conf_str).map_err(From::from)
}

fn run(config: &Config) -> Result {
    let session = BluetoothSession::create_session(None)?;
    let adapter = BluetoothAdapter::init(&session)?;

    adapter.set_powered(true)?;
    if let Some(alias) = &config.adapter_alias {
        adapter.set_alias(alias.clone())?;
    }
    adapter.set_pairable_timeout(0)?;
    adapter.set_pairable(true)?;
    adapter.set_discoverable_timeout(0)?;
    adapter.set_discoverable(true)?;

    info!("Scanning for untrusted devices ...");

    loop {
        for devicename in adapter.get_device_list()? {
            let device = BluetoothDevice::new(&session, devicename);
            if device.is_paired()? && !device.is_trusted()? {
                info!("Enabling trust: {:?}", device);
                device.set_trusted(true)?;
            }
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}
