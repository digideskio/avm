use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use hyper::Client;
use hyper::client::response::Response;
use hyper::header::Connection;
use hyper::status::StatusCode;

fn build_filename(directory: &String, version_str: &String) -> String {
    let filename = format!("v{}.tar.gz", version_str.clone());
    Path::new(directory).join(filename).to_str()
        .unwrap()
        .into()
}

fn write_to_file(destination_path: &String, version: &String, response: &mut Response) -> Result<String, String> {
    let filename = build_filename(&destination_path, &version);
    let mut file_handle = match File::create(filename.clone()) {
        Ok(handle)  => handle,
        Err(_)    => return Err("Failed to create file".to_string())
    };
    let mut archive = Vec::<u8>::new();
    for byte in response.bytes() {
        archive.push(byte.unwrap());
    }
    match file_handle.write_all(&archive[..]) {
        Ok(_)       => {
            Ok(filename)
        },
        Err(_)    => Err("Failed to write file".to_string())
    }
}

pub fn download_source(version: &String, destination_path: &String) -> Result<String, String> {
    let client = Client::new();

    let url = format!("https://nodejs.org/dist/v{version}/node-v{version}-darwin-x64.tar.gz", version=version);
    let mut response = client.get(&*url)
        .header(Connection::close())
        .send().unwrap();

    if response.status == StatusCode::Ok {
        write_to_file(destination_path, &version, &mut response)
    } else {
        return Err(format!("Download failed with HTTP Status {}", response.status));
    }

}
