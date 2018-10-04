use std::fs;
use pbr::ProgressBar;

#[derive(Debug)]
pub struct Header {
    pub file_id: [char; 4],
    pub version: u32,
    pub offset: u32,
    pub archive_flags: u32,
    pub folder_count: u32,
    pub file_count: u32,
    pub total_folder_name_length: u32,
    pub total_file_name_length: u32,
    pub file_flags: u32
}

#[derive(Debug)]
pub struct FolderRecord {
    pub name_hash: u64,
    pub count: u32,
    pub offset: u32
}

#[derive(Debug)]
pub struct FileRecordBlock {
    pub name: Option<String>,
    pub file_records: Vec<FileRecord>
}

impl Clone for FileRecordBlock {

    fn clone(&self) -> FileRecordBlock {
        FileRecordBlock {
            name: self.name.clone(),
            file_records: self.file_records.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.name = source.name.clone();
        self.file_records = source.file_records.clone();
    }
}

#[derive(Debug)]
pub struct FileRecord {
    pub name_hash: u64,
    pub size: u32,
    pub offset: u32
}

impl Clone for FileRecord {

    fn clone(&self) -> FileRecord {
       FileRecord {
           name_hash: self.name_hash.clone(),
           size: self.size.clone(),
           offset: self.offset.clone()
       }
    }

    fn clone_from(&mut self, source: &FileRecord) {
        self.name_hash = source.name_hash.clone();
        self.size = source.size.clone();
        self.offset = source.offset.clone();
    }
}

#[derive(Debug)]
pub struct CompressedFileBlock {
    pub hash: u64,
    pub name: Option<String>,
    pub original_size: u32,
    pub data: Vec<u8>
}

impl Clone for CompressedFileBlock {
    fn clone(&self) -> CompressedFileBlock {
        CompressedFileBlock {
            hash: self.hash.clone(),
            name: self.name.clone(),
            original_size: self.original_size.clone(),
            data: self.data.clone()
        }
    }

    fn clone_from(&mut self, source: &CompressedFileBlock) {
        self.hash = source.hash.clone();
        self.name = source.name.clone();
        self.original_size = source.original_size.clone();
        self.data = source.data.clone();
    }
}

#[derive(Debug)]
pub struct UncompressedFileBlock {
    pub hash: u64,
    pub name: Option<String>,
    pub data: Vec<u8>
}

impl Clone for UncompressedFileBlock {
    fn clone(&self) -> UncompressedFileBlock {
        UncompressedFileBlock {
            hash: self.hash.clone(),
            name: self.name.clone(),
            data: self.data.clone()
        }
    }

    fn clone_from(&mut self, source: &UncompressedFileBlock) {
        self.hash = source.hash.clone();
        self.name = source.name.clone();
        self.data = source.data.clone();
    }
}

#[derive(Debug)]
pub struct BSA {
    pub header: Header,
    pub folder_records: Vec<FolderRecord>,
    pub file_record_blocks: Vec<FileRecordBlock>,
    pub compressed_file_blocks: Vec<CompressedFileBlock>,
    pub uncompressed_file_blocks: Vec<UncompressedFileBlock>,
    pub file_names: Vec<String>
}

impl BSA {

    // Too lazy to figure out how to return one of two types
    fn block_from_hash(&self, hash: u64) -> (Option<CompressedFileBlock>, Option<UncompressedFileBlock>)
    {
        for compressed_block in &self.compressed_file_blocks {
            if compressed_block.hash == hash {
                return (Some(compressed_block.clone()), None);
            }
        }

        for uncompressed_block in &self.uncompressed_file_blocks {
            if uncompressed_block.hash == hash {
                return (None, Some(uncompressed_block.clone()));
            }
        }

        return (None, None)
    }


    pub fn export(&self, export_path: String)
    {
        let mut counter = 0;
        let mut file_counter = 0;
        let count = &self.header.file_count;
        let mut pb = ProgressBar::new(*count as u64);
        pb.format("[=>-]");
        for file_record_block in &self.file_record_blocks {
            let path = format!("{}/{}", export_path, file_record_block.clone().name.unwrap_or(format!("{}", counter)).replace("\\", "/"));
            fs::create_dir_all(&path);
            for file_record in &file_record_block.file_records {

                let file_name = &self.file_names[file_counter as usize];
                let (comp, uncomp) = self.block_from_hash(file_record.name_hash);

                if comp.is_some() {
                    let compressed_block = comp.unwrap();

                    /* TODO: Implement zlib decompression. */

                } else if uncomp.is_some() {
                    let uncompressed_block = uncomp.unwrap();
                    fs::write(format!("{}/{}", path, file_name), uncompressed_block.data);
                } else {
                    println!("SHIT");
                    return;
                }
                file_counter += 1;
                pb.inc();
            }
            counter += 0;
        }
    }

}

pub enum ArchiveFlags {
    NamedDirs,
    NamedFiles,
    DefComp,
    RetainDirs,
    RetainFiles,
    RetainOffsets,
    BigEnd,
    RetainStrings,
    Unknown1,
    Unknown2,
    Unknown3
}

impl ArchiveFlags {

    pub fn val(&self) -> u32
    {
        match *self {
            ArchiveFlags::NamedDirs => 0x1,
            ArchiveFlags::NamedFiles => 0x2,
            ArchiveFlags::DefComp => 0x4,
            ArchiveFlags::RetainDirs => 0x8,
            ArchiveFlags::RetainFiles => 0x10,
            ArchiveFlags::RetainOffsets => 0x20,
            ArchiveFlags::BigEnd => 0x40,
            ArchiveFlags::RetainStrings => 0x80,
            ArchiveFlags::Unknown1 => 0x100,
            ArchiveFlags::Unknown2 => 0x200,
            ArchiveFlags::Unknown3 => 0x400
        }
    }

}