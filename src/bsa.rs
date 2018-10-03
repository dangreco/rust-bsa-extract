use std::path::Path;

#[derive(Debug)]
pub struct Header {
    file_id: [u8; 4],
    version: u32,
    offset: u32,
    archive_flags: u32,
    folder_count: u32,
    file_count: u32,
    total_folder_name_length: u32,
    total_file_name_length: u32,
    file_flags: u32
}

#[derive(Debug)]
pub struct FolderRecord {
    name_hash: u64,
    count: u32,
    offset: u32
}

#[derive(Debug)]
pub struct FileRecordBlock {
    name: Option<String>,
    file_records: Vec<FileRecord>
}

#[derive(Debug)]
pub struct FileRecord {
    name_hash: u64,
    size: u32,
    offset: u32
}

#[derive(Debug)]
pub struct CompressedFileBlock {
    name: Option<String>,
    original_size: u32,
    data: Vec<u8>
}

#[derive(Debug)]
pub struct UncompressedFileBlock {
    name: Option<String>,
    data: Vec<u8>
}

#[derive(Debug)]
pub struct BSA {
    header: Header,
    folder_records: Vec<FolderRecord>,
    file_record_blocks: Vec<FileRecordBlock>,
    compressed_file_blocks: Vec<CompressedFileBlock>,
    uncompressed_file_blocks: Vec<UncompressedFileBlock>
}


pub fn generate_hash(path: String, is_folder: bool) -> u64
{

    let mut hash: u64 = 0;
    let path_len = path.len();
    let mut path_chars = path.chars();
    let mut s: [char; 255] = ['\0'; 255];
    let mut string_as_path = Path::new(path);
    let has_ext = string_as_path.extension() != None;

    return hash;

}