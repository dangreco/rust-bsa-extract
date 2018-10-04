use bsa::*;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub fn ulong(data: &[u8], big_endian: bool) -> Option<u32>
{
    let mut buf = &data[..];
    let res;
    if big_endian {
        res = buf.read_u32::<BigEndian>();
    } else {
        res = buf.read_u32::<LittleEndian>();
    }
    match res {
        Ok(v) => { return Some(v) },
        Err(e) => { return None }
    }
}

pub fn hash(data: &[u8], big_endian: bool) -> Option<u64>
{
    let mut buf = &data[..];
    let res;
    if big_endian {
        res = buf.read_u64::<BigEndian>();
    } else {
        res = buf.read_u64::<LittleEndian>();
    }
    match res {
        Ok(v) => { return Some(v) },
        Err(e) => { return None }
    }
}

pub fn bzstring(data: &[u8]) -> Option<(String, usize)>
{
    let byte_length = data[0];
    let string = &data[1..byte_length as usize];
    let res = String::from_utf8(string.to_vec());
    match res {
        Ok(v) => { return Some((v, byte_length as usize + 1)) },
        Err(e) => { return None }
    }
}

pub fn bstring(data: &[u8]) -> Option<(String, usize)>
{
    let byte_length = data[0];
    let string = &data[1..byte_length as usize + 1];
    let res = String::from_utf8(string.to_vec());
    match res {
        Ok(v) => { return Some((v, byte_length as usize + 1)) },
        Err(e) => { return None }
    }
}

pub fn zstring(data: &[u8]) -> Option<(String, usize)>
{
    let mut buf: Vec<u8> = vec![];
    let mut counter: usize = 0;
    let mut current = data[0];

    while current != 0 {
        buf.push(current);
        counter += 1;
        current = data[counter];
    }
    let res = String::from_utf8(buf);
    match res {
        Ok(v) => {
            let len = &v.len();
            return Some((v, len + 1))
        },
        Err(e) => { return None }
    }
}

fn get_header(d: &Vec<u8>) -> Option<Header>
{
    /* Ensure file as at least 36 bytes to parse header */
    if d.len() < 36 {
        println!("File too small to have header.");
        return None;
    }

    let file_id: [char; 4] = [
        char::from(d[0]),
        char::from(d[1]),
        char::from(d[2]),
        char::from(d[3])
    ];

    let version = ulong(&d[4..8], false);
    let offset = ulong(&d[8..12], false);
    let archive_flags = ulong(&d[12..16], false);
    let folder_count = ulong(&d[16..20], false);
    let file_count = ulong(&d[20..24], false);
    let total_folder_name_length = ulong(&d[24..28], false);
    let total_file_name_length = ulong(&d[28..32], false);
    let file_flags = ulong(&d[32..36], false);

    /* Check constants & ensure non-None's */
    if file_id != ['B', 'S', 'A', '\x00'] {
        println!("Invalid FileId. (Not BSA file)");
        return None;
    }

    if version == None || offset == None || archive_flags == None || folder_count == None
        || file_count == None || total_folder_name_length == None || total_file_name_length == None
        || file_flags == None {
        println!("Could not parse header.");
        return None;
    }

    if version.unwrap() != 103 {
        println!("Invalid BSA version. (Version is not 103)");
        return None;
    }

    if offset.unwrap() != 36 {
        println!("Offset is not 36 bytes. Something's wrong.");
        return None;
    }

    let header = Header {
        file_id,
        version: version.unwrap(),
        offset: offset.unwrap(),
        archive_flags: archive_flags.unwrap(),
        folder_count: folder_count.unwrap(),
        file_count: file_count.unwrap(),
        total_folder_name_length: total_folder_name_length.unwrap(),
        total_file_name_length: total_file_name_length.unwrap(),
        file_flags: file_flags.unwrap()
    };
    return Some(header);
}

fn get_folder_records(d: &Vec<u8>, folder_count: u32, big_endian: bool) -> Vec<FolderRecord>
{
    let mut folder_records: Vec<FolderRecord> = vec![];

    for i in 0..folder_count {

        let s = ((i * 16) + 36) as usize;

        let name_hash = hash(&d[s..s+8], big_endian)
            .expect("Could not parse folder hash.");
        let count = ulong(&d[s+8..s+12], big_endian)
            .expect("Could not parse folder file count.");
        let offset = ulong(&d[s+12..s+16], big_endian)
            .expect("Could not parse folder offset.");

        folder_records.push(FolderRecord {
            name_hash,
            count,
            offset
        });

    }

    return folder_records;
}

fn get_file_record_block(d: &Vec<u8>, folder_record: &FolderRecord, total_file_name_length: u32, named_directories: bool, big_endian: bool) -> FileRecordBlock
{
    let mut real_offset = (folder_record.offset - total_file_name_length) as usize;
    let mut file_records: Vec<FileRecord> = vec![];
    let mut directory_name = None;

    if named_directories {
        let (name, offset) = bzstring(&d[real_offset..])
            .expect("Could not parse file record bzstring.");
        directory_name = Some(name);
        real_offset += offset;
    }

    for i in 0..folder_record.count {

        let s = real_offset + (16 * i) as usize;

        let name_hash = hash(&d[s..s+8], big_endian)
            .expect("Could not parse file record hash.");
        let size = ulong(&d[s+8..s+12], big_endian)
            .expect("Could not parse file record size.");
        let offset = ulong(&d[s+12..s+16], big_endian)
            .expect("Could not parse file record offset.");

        file_records.push(FileRecord {
            name_hash,
            size,
            offset
        });

    }

    return FileRecordBlock {
        name: directory_name,
        file_records
    };
}

fn get_file_names(d: &Vec<u8>, offset: u32, file_count: u32) -> Vec<String>
{
    let mut start = offset as usize;
    let mut file_names: Vec<String> = vec![];

    for i in 0..file_count {

        let (name, offset) = zstring(&d[start..])
            .expect("Could not parse file name zstring.");

        start += offset;
        file_names.push(name);

    }

    return file_names;
}

fn get_compressed_file_block(d: &Vec<u8>, file_record: &FileRecord, ninth: bool, big_endian: bool) -> CompressedFileBlock
{
    let mut start = file_record.offset as usize;
    let mut name = None;
    let end = start + file_record.size as usize;

    if ninth {
        let (path, offset) = bstring(&d[start..])
            .expect("Could not parse compressed bstring.");
        name = Some(path);
        start += offset;
    }

    let original_size = ulong(&d[start..start+4], big_endian)
        .expect("Could not parse compressed original size.");

    return CompressedFileBlock {
        hash: file_record.name_hash,
        name,
        original_size,
        data: d[start+4..end].to_vec()
    };
}

fn get_uncompressed_file_block(d: &Vec<u8>, file_record: &FileRecord, ninth: bool, big_endian: bool) -> UncompressedFileBlock
{
    let mut start = file_record.offset as usize;
    let mut name = None;
    let end = start + file_record.size as usize;

    if ninth {
        let (path, offset) = bstring(&d[start..])
            .expect("Could not parse uncompressed bstring.");
        name = Some(path);
        start += offset;
    }

    return UncompressedFileBlock {
        hash: file_record.name_hash,
        name,
        data: d[start..end].to_vec()
    };
}

pub struct BSAParser {
    data: Vec<u8>
}

impl BSAParser {

    pub fn new(data: Vec<u8>) -> BSAParser
    {
        return BSAParser {
            data
        }
    }

    pub fn parse(&mut self) -> Option<BSA>
    {
        let d = &self.data;
        let header = get_header(d).expect("Could not parse header.");

        let mut file_record_blocks: Vec<FileRecordBlock> = vec![];
        let mut compressed_file_blocks: Vec<CompressedFileBlock> = vec![];
        let mut uncompressed_file_blocks: Vec<UncompressedFileBlock> = vec![];

        let mut name_block_start: u32 = 0;

        let big_endian = header.archive_flags & ArchiveFlags::BigEnd.val() != 0;
        let named_dirs = header.archive_flags & ArchiveFlags::NamedDirs.val() != 0;
        let ninth = false;
        let def_comp = header.archive_flags & ArchiveFlags::DefComp.val() != 0;
        let total_file_name_length = header.total_file_name_length;

        let folder_records = get_folder_records(d, header.folder_count, big_endian);

        for folder_record in &folder_records {

            let file_record_block = get_file_record_block(d, folder_record, total_file_name_length, named_dirs, big_endian);

            for file_record in &file_record_block.file_records {

                /* Check if compressed */
                let mut compressed = false;
                let set = file_record.size & 1<<30 != 0;

                if set {
                    compressed = !def_comp;
                }

                if compressed {

                    let compressed_file_block = get_compressed_file_block(d, file_record, ninth, big_endian);
                    compressed_file_blocks.push(compressed_file_block);

                } else {

                    let uncompressed_file_block = get_uncompressed_file_block(d, file_record, ninth, big_endian);
                    uncompressed_file_blocks.push(uncompressed_file_block);

                }

            }

            {
                let block_clone = file_record_block.clone();
                let folder_name = block_clone.name.unwrap_or("".to_owned());
                let mut name_len = folder_name.len() as u32;
                if name_len > 0 {
                    name_len += 2; // For zero byte & byte length
                }
                let block_end = (folder_record.offset - total_file_name_length) + name_len + (16 * folder_record.count);
                name_block_start = block_end;
            }

            file_record_blocks.push(file_record_block);
        }

        let file_names = get_file_names(d, name_block_start, header.file_count);

        return Some(BSA {
            header,
            folder_records,
            file_record_blocks,
            compressed_file_blocks,
            uncompressed_file_blocks,
            file_names
        });
    }

}
