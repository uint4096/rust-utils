use std::fs::{Metadata, DirEntry};
use std::os::unix::prelude::{PermissionsExt, MetadataExt};
use std::time::SystemTime;
use std::{io, fs};
use std::env::args;
use users::{get_user_by_uid, get_group_by_gid};
mod elements;
mod utils;
use elements::permission::Permission;
use utils::time;

struct Entity {
  is_file: bool,
  permissions: Permission,
  owner: String,
  group: String,
  last_modified: SystemTime,
  size: u64,
  links: u64,
  name: String
}

impl Entity {
  fn new (name: &str, metadata: io::Result<Metadata>) -> io::Result<Entity> {
    let meta = metadata?;
    let owner = get_user_by_uid(meta.uid())
      .unwrap()
      .name()
      .to_str()
      .unwrap()
      .to_owned();
    
    let group = get_group_by_gid(meta.gid())
      .unwrap()
      .name()
      .to_str()
      .unwrap()
      .to_owned(); 

    Ok(Entity {
      permissions: Permission::get_permission(meta.permissions().mode()),
      is_file: meta.is_file(),
      owner,
      group,
      last_modified: meta.modified()?,
      links: meta.nlink(),
      size: meta.size(),
      name: name.to_owned(),
    })
  }

  fn format_rows(mut rows: Vec<Entity>) -> Vec<String> {
    let format_with_spaces = | max_len: usize, elem: &String | {
      let num_spaces = max_len - elem.len();
      let spaces = (0..num_spaces).fold(String::from(""), | mut s, _ | { s.push(' '); s });
      let mut final_str = String::from(elem);
      final_str.push_str(&spaces);
      final_str
    };

    let (user_max_len, group_max_len, size_max_len) = rows
      .iter().fold((0, 0, 0), | mut tup, elem | { 
        if elem.owner.len() > tup.0 { tup.0 = elem.owner.len(); }
        if elem.group.len() > tup.1 { tup.1 = elem.group.len(); }
        if elem.size.to_string().len() > tup.2 { tup.2 = elem.size.to_string().len(); }
        tup
      });

    rows.sort_by(|a, b| b.is_file.cmp(&a.is_file));
    rows.iter()
      .map(| row | format!(
        "{}{} {} {} {} {} {} {}",
        if row.is_file { "-" } else { "d" },
        row.permissions.get_str_permissions(),
        row.links,
        format_with_spaces(user_max_len, &row.owner),
        format_with_spaces(group_max_len, &row.group),
        format_with_spaces(size_max_len, &row.size.to_string()),
        time::format_system_time(row.last_modified),
        if row.is_file { format!("{}", row.name) } else { format!("\x1b[31;1m{}\x1b[0m", row.name) }
      ))
      .collect()
  }
}


fn get_entites(dir: &str, ignore_hidden: bool) -> io::Result<()> {
  let files = fs::read_dir(dir).expect("Error while reading directory!");
  /*
   *@todo: I'd rather have an iterator here. How do we get an iterator of the same type
   * here for both if and else blocks?
   */
  let dir_entries: Vec<io::Result<DirEntry>> = if ignore_hidden {
    files.filter(|f| {
      // Q. Why does as_ref magically fix everything here?
      let file = f.as_ref().expect("Corrupt Entry");

      if ignore_hidden {
        !file.file_name().to_str().unwrap().starts_with('.')
      } else { true }
    }).collect()
  } else {
    files.filter(|_| true).collect()
  };

  let entries = dir_entries.iter()
    .map(| file | {
      let file_entry = file.as_ref().expect("Corrupt entry!");
      let metadata = file_entry.metadata();
      let file_name = file_entry.file_name().to_str().unwrap().to_owned();
      match Entity::new(&file_name, metadata) {
        Ok(e) => { e }
        Err(_) => { panic!("Unable to create entity for file {}", file_name) }
      }
    })
    .collect();

  let _ = Entity::format_rows(entries)
    .iter()
    .for_each(| entry | println!("{}", entry));

  Ok(())
}

fn main() {
  let ls_args: Option<String> = args().skip(1).next();
  let agruments: Vec<char> = match ls_args {
      Some(str) => {
          if str.starts_with('-') {
              str.chars().collect()
          } else {
              vec![]
          }
      }
      None => {
          vec![]
      }
  };

  let ignore_hidden = !agruments.contains(&'a');

  let _ = get_entites("/home/abhishek", ignore_hidden);
}
