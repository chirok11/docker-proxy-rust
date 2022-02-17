mod proxy;

use std::env::args;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

#[macro_use] extern crate log;

#[derive(Debug)]
enum Proto {
    TCP,
    UDP,
    SCTP,
    Unknown
}

impl Default for Proto {
    fn default() -> Self {
        Proto::Unknown
    }
}

#[derive(Default, Debug)]
struct DockerProxy {
    proto: Proto,
    host: String,
    port: u16,
    target: String,
    target_port: u16
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    println!("Hello, world!");

    let mut config = DockerProxy::default();
    debug!("parsing command line");
    let mut n = 0;
    for arg in args() {
        debug!("parsing {}", arg);
        n += 1;
        match arg.as_str() {
            "-proto" => match args().nth(n).unwrap().as_str() {
                "tcp" => config.proto = Proto::TCP,
                "udp" => config.proto = Proto::UDP,
                "sctp" => config.proto = Proto::SCTP,
                v => { panic!("unsupported protocol {}", v) }
            },
            "-host-ip" => config.host = args().nth(n).unwrap(),
            "-host-port" => config.port = args().nth(n).unwrap().parse().unwrap(),
            "-container-ip" => config.target = args().nth(n).unwrap(),
            "-container-port" => config.target_port = args().nth(n).unwrap().parse().unwrap(),
            _ => {}
        }
    }
    debug!("{:?}", config);
    debug!("opening file fd 3, signal-parent");
    // let mut f = unsafe { File::from_raw_fd(3) };
    // f.write(b"0\n").unwrap();
    match config.proto {
        Proto::TCP => {
            let v = proxy::tcp_listen_proxy(config.host, config.port, config.target, config.target_port).await;
            debug!("result: {:?}", v);
        }
        _ => unimplemented!()
    }
}

#[test]
fn test_docker_parse_cmdline() {

}