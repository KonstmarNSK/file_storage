use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct FileInfo {
    inode: u64,
    start_pos: u64,
    bytes_to_read: u64,
}

async fn index(file_info: web::Query<FileInfo>) -> impl Responder {
    if 1 != file_info.inode {
        return HttpResponse::Ok().body("something went wrong");
    };

    return match read_file(file_info.start_pos, file_info.bytes_to_read).await {
        Ok(vec) => {
            println!("read: {:?}", &vec);
            HttpResponse::Ok().body(vec)
        }
        _ => HttpResponse::Ok().body("no data")
    };
}

async fn read_file(start_pos: u64, bytes_to_read: u64) -> std::io::Result<Vec<u8>> {
    println!("reading {} bytes, starting with {}", bytes_to_read, start_pos + 1);

    let mut file = File::open("foo.bin")?;
    file.seek(SeekFrom::Start(start_pos))?;

    let mut buf = vec![0; bytes_to_read as usize];
    let read_bytes = file.take(bytes_to_read).read(&mut buf)?;

    Ok(Vec::from(&buf[0..read_bytes]))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/get-file-bytes", web::get().to(index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

