use rutils::{file::reader::Reader, utils::errors::UtilResult};
use std::str;

fn main() -> UtilResult<()> {
    const DEFAULT_CHUNK_SIZE: usize = 1000;
    const DEFAULT_LINES: usize = 20;

    let mut reader = Reader::open_file(String::from("./test.txt"))?;
    let chunk_size: usize = if reader.size < DEFAULT_CHUNK_SIZE {
        reader.size
    } else {
        DEFAULT_CHUNK_SIZE
    };

    last_x_lines(&mut reader, chunk_size, DEFAULT_LINES)?;
    follow(&mut reader)?;
    Ok(())
}

fn last_x_lines<'a> (reader: &mut Reader, mut chunk_size: usize, lines: usize) -> UtilResult<()> {
    let file_size = reader.size;
    let mut offset = if file_size > chunk_size {
        file_size - chunk_size
    } else {
        0
    };

    let mut buf = vec![0u8; chunk_size as usize];
    let mut lines_count: usize = 0;
    let mut bytes_processed: usize = 0;
    let mut tail_output: Vec<String> = vec![];

    while lines_count <= lines && bytes_processed < file_size {
        reader.read_from(offset as u64, &mut buf)?;
        let buf_str = str::from_utf8(&buf);
        tail_output.push(buf_str?.to_owned());

        let current_size = buf_str?.matches('\n').count();
        lines_count += current_size;
        bytes_processed += buf_str?.len();

        let temp_offset = offset;
        offset -= if offset >= chunk_size {
            chunk_size
        } else {
            offset
        };
        chunk_size = if offset == 0 { temp_offset } else { chunk_size };

        buf = vec![0u8; chunk_size];
    }

    tail_output.reverse();
    let additional_lines = if lines_count < lines {
        0
    } else {
        lines_count - lines
    };
    let tail_output = tail_output.concat();
    let mut lines = tail_output.split('\n');
    for _ in 0..additional_lines {
        lines.next();
    }

    let out = lines.fold(String::new(), |mut acc, val| {
        acc.push_str(&format!("{val}\n"));
        acc
    });

    print!("{out}");
    Ok(())
}

fn follow<'a> (reader: &mut Reader) -> UtilResult<()> {
    loop {
        let mut follow_offset = reader.size;
        loop {
            let line = reader.read_line_from(follow_offset as u64)?;
            if line.len() > 0 {
                println!("{line}");
                follow_offset += line.len() + 1;
            }
        }
    }
}
