// util

use std::{
    time::Duration,
    io::{Read, Write},
};
use std::net::{
    TcpStream, ToSocketAddrs
};
use native_tls::TlsConnector;
use tempfile::NamedTempFile;

pub fn get_indexed_wrapped<'a: 'b, 'b>
    (lines: &Vec<&'a str>, width: usize) -> Vec<(usize, &'b str)> 
{
    let mut wrapped: Vec<(usize, &'b str)> = vec![];

    for (i, l) in lines.iter().enumerate() {
        let v = get_wrapped(l, width);
        for s in v.iter() {
            wrapped.push((i, s));
        }
    }
    wrapped
}

pub fn get_wrapped<'a: 'b, 'b>
    (line: &'a str, width: usize) -> Vec<&'b str> 
{
    let mut wrapped: Vec<&str> = vec![];
    let mut start  = 0;
    let mut end    = width;
    let     length = line.len();

    while end < length {
        let longest = &line[start..end];
        match longest.rsplit_once(' ') {
            Some((a, b)) => {
                let shortest = match a.len() {
                    0 => b,
                    _ => a,
                };
                wrapped.push(shortest);
                start += shortest.len();
                end    = start + width;
            }
            None => {
                wrapped.push(longest);
                start = end;
                end  += width;
            }
        }
    }
    if start < length {
        wrapped.push(&line[start..length]);
    }
    wrapped
}

pub fn get_data(url: &url::Url) -> Result<(String, String), String> 
{
    let host = url.host_str().unwrap_or("");
    let urlf = format!("{}:1965", host);

    // get connector
    let connector = TlsConnector::builder()
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true)
        .build()
        .or_else(
            |e| Err(format!("Could not connect to {}\n{}", urlf, e))
        )?;

    // get socket address iterator
    let mut addrs_iter = urlf.to_socket_addrs()
        .or_else(
            |e| Err(format!("Could not connect to {}\n{}", urlf, e))
        )?;

    // get socket address from socket address iterator
    let Some(socket_addr) = addrs_iter.next() 
        else {
            return Err(format!("Could not connect to {}", urlf))
        };

    // get tcp stream from socket address
    let tcpstream = TcpStream::connect_timeout
        (&socket_addr, Duration::new(10, 0))
        .or_else(
            |e| Err(format!("Could not connect to {}\n{}", urlf, e))
        )?;

    // get stream from tcp stream
    let mut stream = connector.connect(&host, tcpstream) 
        .or_else(
            |e| Err(format!("Could not connect to {}\n{}", urlf, e))
        )?;

    // write url to stream
    stream.write_all(format!("{}\r\n", url).as_bytes())
        .or_else(
            |e| Err(format!("Could not write to {}\n{}", url, e))
        )?;

    // initialize response vector
    let mut response = vec![];

    // load response vector from stream
    stream.read_to_end(&mut response)
        .or_else(
            |e| Err(format!("Could not read {}\n{}", url, e))
        )?;

    // find clrf in response vector
    let Some(clrf_idx) = find_clrf(&response) 
        else {
            return Err("Could not find the clrf".to_string())
        };

    // separate response from content
    let content = response.split_off(clrf_idx + 2);

    // convert to String
    let content  = String::from_utf8_lossy(&content).to_string();
    let response = String::from_utf8_lossy(&response).to_string();

    Ok((response, content))
}

pub fn download(content: String) 
{
    let path = write_tmp_file(content.into_bytes());
    open::that(path).unwrap();
}

fn write_tmp_file(content: Vec<u8>) -> std::path::PathBuf 
{
    let mut tmp_file = NamedTempFile::new().unwrap();
    tmp_file.write_all(&content).unwrap();
    let (_file, path) = tmp_file.keep().unwrap();
    path
}

fn find_clrf(data: &[u8]) -> Option<usize> 
{
    let clrf = b"\r\n";
    data.windows(clrf.len()).position(|window| window == clrf)
}
