use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}, 
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {handle(stream)});
        
    }

}

fn handle(mut stream : TcpStream) {

    let buffer_reader = BufReader::new(&mut stream);
    let request_page = buffer_reader.lines().next().unwrap().unwrap();
    
    let (status_line, file_path) = match request_page.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "f_server.html"),
        "GET /test HTTP/1.1" => ("HTTP/1.1 200 OK", "test.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "test.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "error.html"),
    };

    let content = fs::read_to_string(file_path).unwrap();
    let length = content.len();

    let resp = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(resp.as_bytes()).unwrap();

    
        
    
}
