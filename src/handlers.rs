use tokio::prelude::*;
use tokio::fs::File;
use std::io::{Read, Seek, SeekFrom};

use actix_web::{HttpResponse, Responder, web};

use structs::{INode, NodeInfo, NodeType, ReadFileInfo, SearchTargetInfo};

use super::structs;

pub(crate) fn add_fs_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("read-file", web::get().to(read_file_handler))
            .route("read-dir", web::get().to(get_dir_content_handler))
            .route("find", web::get().to(find))
            .route("get-info", web::get().to(get_info_handler))
    );
}


async fn read_file_handler(file_info: web::Query<ReadFileInfo>) -> impl Responder {
    if 2 != file_info.inode {
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

async fn find(target_info: web::Query<SearchTargetInfo>) -> impl Responder {
    return match target_info.into_inner() {
        SearchTargetInfo { parent_inode: 1, target_name: a } =>
            serde_json::to_string(&get_info(2).await).unwrap(),

        _ => "No such file or directory".to_string()
    };
}


async fn get_info_handler(data: web::Query<INode>) -> impl Responder {
    return match data.inode {
        2 => serde_json::to_string(&get_info(1).await).unwrap(),
        _ => "Err indoe worng".to_string()
    };
}

async fn get_info(inode: u64) -> Box<NodeInfo> {
    Box::new(NodeInfo {
        name: "file_name".to_string(),
        inode,
        kind: NodeType::FILE,
        size: 30,
    })
}

async fn get_dir_content_handler(dir_inode: web::Query<INode>) -> impl Responder{
    serde_json::ser::to_string(
        &vec![
            NodeInfo{
                name: "file_name".to_string(),
                inode: 2,
                kind: NodeType::FILE,
                size: 30,
            },

            NodeInfo{
                name: ".".to_string(),
                inode: 1,
                kind: NodeType::DIRECTORY,
                size: 30,
            },

            NodeInfo{
                name: "..".to_string(),
                inode: 1,
                kind: NodeType::DIRECTORY,
                size: 30,
            }
        ]
    )
}

async fn read_file(start_pos: u64, bytes_to_read: u64) -> std::io::Result<Vec<u8>> {
    println!("reading {} bytes, starting with {}", bytes_to_read, start_pos + 1);

    let mut file = File::open("foo.bin").await?;
    println!("opened");

    file.seek(SeekFrom::Start(start_pos)).await?;
    println!("moved cursor");

    let mut buf = vec![0; bytes_to_read as usize];
    let read_bytes = file.read(&mut buf).await?;

    println!("read_bytes: {}, buf: {:?}", read_bytes, &buf);

    Ok(Vec::from(&buf[0..read_bytes]))
}
