#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]

use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
use embassy_net::{ConfigV4, ConfigV6, Ipv6Cidr, StaticConfigV6};
use rs_matter::pairing::code::compute_pairing_code;
use rs_matter::error::Error;
use rs_matter::mdns::{MdnsService, MdnsRunBuffers};
use rs_matter::transport::core::RunBuffers;
use static_cell::make_static;

use core::borrow::Borrow;
use embassy_net::{Config, Stack, StackResources};
use embassy_time::{Duration, Timer};
use embedded_svc::wifi::Wifi;
use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_backtrace as _;
use esp_println::println;
use esp_wifi::wifi::{WifiController, WifiDevice, WifiEvent, WifiMode, WifiState};
use esp_wifi::{current_millis, initialize, EspWifiInitFor};
use hal::clock::{ClockControl, CpuClock};
use hal::systimer::SystemTimer;
use hal::{embassy, Rng};
use hal::{peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};
use log::info;

use rs_matter::data_model::cluster_basic_information::BasicInfoConfig;
use rs_matter::data_model::device_types::DEV_TYPE_ON_OFF_LIGHT;
use rs_matter::data_model::objects::{Endpoint, HandlerCompat, Metadata, Node, NonBlockingHandler};
use rs_matter::data_model::system_model::descriptor;
use rs_matter::data_model::{cluster_on_off, root_endpoint};
use rs_matter::secure_channel::spake2p::VerifierData;
use rs_matter::{CommissioningData, Matter};
use no_std_net::{Ipv4Addr, Ipv6Addr};
use smoltcp::wire::Ipv6Address;

mod dev_attr;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

const SSID: &str = "Nope";
const PASSWORD: &str = "uh oh";

macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        let (x,) = STATIC_CELL.init(($val,));
        x
    }};
}

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

fn init_heap() {
    // we need some heap memory unfortunately
    const HEAP_SIZE: usize = 8 * 1024;

    extern "C" {
        static mut _heap_start: u32;
    }

    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    init_heap();

    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock160MHz).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let systimer = SystemTimer::new(peripherals.SYSTIMER);
    let inited = initialize(
        EspWifiInitFor::Wifi,
        systimer.alarm0,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    // Connect to wifi
    let (wifi, _) = peripherals.RADIO.split();
    let (wifi_interface, controller) = esp_wifi::wifi::new_with_mode(&inited, wifi, WifiMode::Sta);

    embassy::init(&clocks, timer_group0.timer0);

    let config = Config {
        ipv4: ConfigV4::Dhcp(Default::default()),
        ipv6: ConfigV6::Static(StaticConfigV6 {
            address: Ipv6Cidr::new(
                // TODO: hard-coded link local address
                Ipv6Address::new(0xfe80, 0, 0, 0, 0x6255, 0xf9ff, 0xfec0, 0xfdbc),
                64,
            ),
            gateway: Some(Ipv6Address::new(0xfe80, 0, 0, 0, 0, 0, 0, 0)),
            dns_servers: heapless::Vec::new(),
        }),
    };

    let seed = 1234; // very random, very secure seed

    // Init network stack
    let stack = &*singleton!(Stack::new(
        wifi_interface,
        config,
        singleton!(StackResources::<3>::new()),
        seed
    ));

    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(connection(controller)).ok();
        spawner.spawn(net_task(&stack)).ok();
        spawner.spawn(task(&stack)).ok();
    })
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.get_capabilities());
    loop {
        match esp_wifi::wifi::get_wifi_state() {
            WifiState::StaConnected => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = Configuration::Client(ClientConfiguration {
                ssid: SSID.into(),
                password: PASSWORD.into(),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            println!("Starting wifi");
            controller.start().await.unwrap();
            println!("Wifi started!");
        }
        println!("About to connect...");

        match controller.connect().await {
            Ok(_) => println!("Wifi connected!"),
            Err(e) => {
                println!("Failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<WifiDevice<'static>>) {
    stack.run().await
}

#[embassy_executor::task]
async fn task(stack: &'static Stack<WifiDevice<'static>>) {
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    let mut ipv4_addr_octets = [0u8; 4];
    let mut ipv6_addr_octets = [0u8; 16];

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            ipv4_addr_octets.copy_from_slice(config.address.address().as_bytes());
            // TODO hardcoded link local address for now - you will need to change it!
            ipv6_addr_octets.copy_from_slice(&[
                0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0x62, 0x55, 0xf9, 0xff, 0xfe, 0xc0, 0xfd, 0xbc,
            ]);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Got IPv4: {:?}", &ipv4_addr_octets);
    println!("Got IPv6: {:x?}", &ipv6_addr_octets);

    run_matter(
        stack,
        Ipv4Addr::from(ipv4_addr_octets),
        Some(Ipv6Addr::from(ipv6_addr_octets)),
    )
    .await
    .unwrap();
}

async fn run_matter(
    stack: &'static Stack<WifiDevice<'static>>,
    ipv4_addr: Ipv4Addr,
    ipv6_addr: Option<Ipv6Addr>,
) -> Result<(), Error> {
    info!(
        "Matter required memory: mDNS={}, Matter={}, MdnsBuffers={}, RunBuffers={}",
        core::mem::size_of::<MdnsService>(),
        core::mem::size_of::<Matter>(),
        core::mem::size_of::<MdnsRunBuffers>(),
        core::mem::size_of::<RunBuffers>(),
    );

    let dev_det = &*make_static!(BasicInfoConfig {
        vid: 0xFFF1,
        pid: 0x8000,
        hw_ver: 2,
        sw_ver: 1,
        sw_ver_str: "1",
        serial_no: "aabbccdd",
        device_name: "OnOff Light",
    });

    let dev_att = &*make_static!(dev_attr::HardCodedDevAtt::new());

    let mdns = &*make_static!(MdnsService::new(
        0,
        "matter-demo",
        ipv4_addr.octets(),
        ipv6_addr.map(|ip| (ip.octets(), 0)),
        dev_det,
        rs_matter::MATTER_PORT,
    ));

    let matter = &*make_static!(Matter::new(
        // vid/pid should match those in the DAC
        dev_det,
        dev_att,
        mdns,
        epoch,
        matter_rand,
        rs_matter::MATTER_PORT,
    ));

    let comm_data = CommissioningData {
        // TODO: Hard-coded for now
        verifier: VerifierData::new_with_pw(123456, *matter.borrow()),
        discriminator: 250,
    };
    
    let pairing_code = compute_pairing_code(&comm_data);

    println!("Pairing code: {}", pairing_code);

    let handler = &*make_static!(HandlerCompat(handler(matter)));

    let mut buffers = make_static!(RunBuffers::new());

    let fut = matter.run(
        stack,
        &mut buffers,
        comm_data,
        handler,
    );

    info!(
        "Future initialized, memory size={}",
        core::mem::size_of_val(&fut)
    );
    info!("Starting Matter...");

    fut.await?;

    Ok(())
}

const NODE: Node<'static> = Node {
    id: 0,
    endpoints: &[
        root_endpoint::endpoint(0),
        Endpoint {
            id: 1,
            device_type: DEV_TYPE_ON_OFF_LIGHT,
            clusters: &[descriptor::CLUSTER, cluster_on_off::CLUSTER],
        },
    ],
};

fn handler<'a>(matter: &'a Matter<'a>) -> impl Metadata + NonBlockingHandler + 'a {
    (
        NODE,
        root_endpoint::handler(0, matter)
            .chain(
                1,
                descriptor::ID,
                descriptor::DescriptorCluster::new(*matter.borrow()),
            )
            .chain(
                1,
                cluster_on_off::ID,
                cluster_on_off::OnOffCluster::new(*matter.borrow()),
            ),
    )
}

fn epoch() -> core::time::Duration {
    core::time::Duration::from_millis(current_millis())
}

fn matter_rand(buffer: &mut [u8]) {
    for b in buffer.iter_mut() {
        *b = unsafe { esp_wifi::wifi::rand() as u8 };
    }
}
