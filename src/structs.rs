use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct ReadFileInfo {
    pub inode: u64,
    pub start_pos: u64,
    pub bytes_to_read: u64,
}

#[derive(Deserialize)]
pub(crate) struct SearchTargetInfo {
    pub parent_inode: u64,
    pub target_name: String,
}

#[derive(Serialize)]
pub(crate) struct NodeInfo {
    pub name: String,
    pub inode: u64,
    pub kind: NodeType,
    pub size: u64,
}

#[derive(Serialize)]
pub(crate) enum NodeType {
    FILE,
    DIRECTORY,
}


#[derive(Deserialize)]
pub(crate) struct INode{
    pub inode: u64,
}