extern crate nix;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_pty_process;

use tokio_core::reactor::Core;

use std::process::Command;
use tokio_pty_process::CommandExt;
use futures::future::Future;
use tokio_io::codec::BytesCodec;
use tokio_io::AsyncRead;
use futures::Stream;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let master = tokio_pty_process::AsyncPtyMaster::open(&handle).unwrap();


    let child = Command::new("echo").arg("hello").arg("world").spawn_pty_async(&master, &handle);
    let futureS = master.framed(BytesCodec::new())
        .for_each(|b|{
            println!("{}", String::from_utf8_lossy(b.as_ref()));
            Ok(())
        })
        .map_err(|e|println!("re:{}",e));

    let futureE = child.expect("failed to spawn")
        .map(|status| println!("exit status: {}", status))
        .map_err(|e| panic!("failed to wait for exit: {}", e));


    let future = futureE.select(futureS);

    core.run(future);

}
