use opentls::{
    sync::{TlsAcceptor, TlsStream},
    Identity,
};
use std::{
    fs::File,
    io::Read,
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

fn main() {
    let mut file = File::open("identity.pfx").unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();
    let pkcs12 = Identity::from_pkcs12(&pkcs12, "hunter2").unwrap();

    let acceptor = TlsAcceptor::new(pkcs12).unwrap();
    let acceptor = Arc::new(acceptor);

    let listener = TcpListener::bind("0.0.0.0:8443").unwrap();

    fn handle_client(_stream: TlsStream<TcpStream>) {
        // ...
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(stream);
                });
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}
