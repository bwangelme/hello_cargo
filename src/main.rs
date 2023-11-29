mod tutorial;

// use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
// use thrift::protocol::{TInputProtocol, TOutputProtocol};
use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport};
use thrift::transport::{TIoChannel, TTcpChannel};
use std::{thread,time};

use tutorial::{CalculatorSyncClient, TCalculatorSyncClient};

fn main() {
    match run() {
        Ok(()) => println!("client ran successfully"),
        Err(e) => {
            println!("client failed with {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> thrift::Result<()> {
    //
    // build client
    //

    println!("connect to server on 127.0.0.1:9090");
    let mut c = TTcpChannel::new();
    c.open("127.0.0.1:9090")?;

    let (i_chan, o_chan) = c.split()?;

    let i_prot = TBinaryInputProtocol::new(
        TFramedReadTransport::new(i_chan), true
    );
    let o_prot = TBinaryOutputProtocol::new(
        TFramedWriteTransport::new(o_chan), true
    );

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);


    loop {
        let res = client.add(7, 8)?;
        println!("multiplied 7 and 8, got {}", res);
        thread::sleep(time::Duration::from_secs(10));
    }
}
