use std::{fs::File, io::{Seek, SeekFrom, Read}, str};
use rutils::utils::errors::{UtilResult};
fn main() -> UtilResult<'static, ()> {
    const DEFAULT_CHUNK_SIZE: usize = 1000;
    const DEFAULT_LINES: usize = 20;

    let mut file = File::open("./Cargo.lock")?;
    let file_size: usize = file.metadata()?.len() as usize;
    let mut chunk_size: usize = if file_size < DEFAULT_CHUNK_SIZE { file_size } else { DEFAULT_CHUNK_SIZE };
    let mut offset = if file_size > chunk_size { file_size - chunk_size } else { 0 };

    let mut buf = vec![0u8; chunk_size as usize]; 
    let mut lines_count: usize = 0;
    let mut bytes_processed: usize = 0;
    let mut tail_output: Vec<String> = vec![];

    loop {
        file.seek(SeekFrom::Start(offset as u64))?;
        file.read_exact(&mut buf)?;

        let buf_str = str::from_utf8(&mut buf);
        tail_output.push(buf_str?.to_owned());

        let current_size = buf_str?.matches('\n').count();
        lines_count += current_size;
        bytes_processed += buf_str?.len();

        let temp_offset = offset;
        offset -= if offset >= chunk_size { chunk_size } else { offset };
        chunk_size = if offset == 0 { temp_offset } else { chunk_size };

        buf = vec![0u8; chunk_size];

        if lines_count > DEFAULT_LINES || bytes_processed >= file_size {
            break;
        }
    }

    tail_output.reverse();
    let tail_output = tail_output.concat();
    let mut lines = tail_output.lines();

    let total_lines = if lines_count < DEFAULT_LINES { 0 } else { lines_count - DEFAULT_LINES };
    for _ in 0..total_lines { lines.next(); }

    let out = lines.fold(String::new(), |mut acc, val| {
        acc.push_str(&format!("{val}\n"));
        acc
    });

    println!("{out}");
    Ok(())
}
