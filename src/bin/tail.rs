use std::{fs::File, io::{Error, Seek, SeekFrom, Read}, str};
fn main() -> Result<(), Error> {
    let mut file = File::open("./Cargo.toml").expect("L1");

    let size = file.metadata().expect("L2").len();
    let mut chunk_size: u64 = 40;
    let mut offset = if size > chunk_size { size - chunk_size } else { 0 };

    let mut buf = vec![0u8; chunk_size as usize]; 
    let mut lines_count = 0;
    let mut tail_output: Vec<String> = vec![];
    loop {
        file.seek(SeekFrom::Start(offset));
        file.read_exact(&mut buf);

        let buf_str = str::from_utf8(&mut buf).unwrap();
        let size: usize = buf_str.matches('\n').count();
        lines_count += size;

        let temp_offset = offset;
        offset -= if offset >= chunk_size { chunk_size } else { offset };
        chunk_size = if offset == 0 { temp_offset } else { chunk_size };
        tail_output.push(buf_str.to_owned());
        buf = vec![0u8; chunk_size as usize];
        if lines_count > 5 {
            break;
        }
    }

    tail_output.reverse();
    let binding = tail_output.concat();
    let mut lines = binding.lines();
    for _ in 0..lines_count-5 {
        lines.next();
    }

    let out = lines.fold(String::new(), |mut acc, val| {
        acc.push_str(&format!("{val}\n"));
        acc
    });

    println!("{out}");

    Ok(())
}
