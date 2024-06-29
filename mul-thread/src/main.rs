use std::{
    fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}, thread, time::Duration
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // handle_request(stream);

        pool.excute(|| handle_request(stream));
    }

}

#[allow(unused)]
fn handle_request(mut stram: TcpStream) {
    let request = BufReader::new(&stram);
    let request_line = request.lines().next().unwrap().unwrap();

    // let http_request: Vec<_> = request
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("{:#?}", http_request);


    // //response
    // let header = "HTTP/1.1 200 OK\r\n";

    // let contents = fs::read_to_string("hello.html").unwrap();

    // let length = contents.len();

    // let response = 
    //     format!("{header}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // stram.write_all(response.as_bytes()).unwrap();

    let (header, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")}
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let length = contents.len();

    let response = 
        format!("{header} \r\nContentLength: {length} \r\n {contents}");

    stram.write_all(response.as_bytes()).unwrap();

}






