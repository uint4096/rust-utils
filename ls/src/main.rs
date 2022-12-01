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
  fn new (name: &str, metadata: Metadata) -> io::Result<Entity> {
    let owner = get_user_by_uid(metadata.uid())
      .unwrap()
      .name()
      .to_str()
      .unwrap()
      .to_owned();
    
    let group = get_group_by_gid(metadata.gid())
      .unwrap()
      .name()
      .to_str()
      .unwrap()
      .to_owned(); 

    Ok(Entity {
      permissions: Permission::get_permission(metadata.permissions().mode()),
      is_file: metadata.is_file(),
      owner,
      group,
      last_modified: metadata.modified()?,
      links: metadata.nlink(),
      size: metadata.size(),
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

fn get_entries (dir: &str, ignore_hidden: bool) -> Vec<io::Result<DirEntry>> {
  let files = fs::read_dir(dir).expect("Error while reading directory!");
  /*
   *@todo: I'd rather have an iterator here. How do we get an iterator of the same type
   * here for both if and else blocks?
   */
  let dir_entries: Vec<io::Result<DirEntry>> = if ignore_hidden {
    files.filter(|f| {
      // Q. Why does as_ref magically fix everything here?
      let file = f.as_ref().expect("Corrupt Entry");
      if ignore_hidden { !file.file_name().to_str().unwrap().starts_with('.')}
      else { true }
    }).collect()
  } else {
    files.filter(|_| true).collect()
  };

  dir_entries
}

fn print_list(dir_entries: Vec<io::Result<DirEntry>>) -> () {
  let entries = dir_entries.iter()
    .map(| file | {
      let file_entry = file.as_ref().expect("Corrupt entry!");
      let metadata = file_entry.metadata().expect("Unable to get metadata!");
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
}

fn print_names (dir_entries: Vec<io::Result<DirEntry>>) -> () {
  let _ = dir_entries.iter()
    .for_each(| entry | {
      let file = entry.as_ref().expect("Corrupt entry!");
      let metadata = file.metadata().expect("Unable to get metadata!");
      let file_name = file.file_name().to_str().unwrap().to_owned();
      if metadata.is_file() {
        print!("{} ", file_name);
      } else {
        print!("\x1b[31;1m{}\x1b[0m ", file_name);
      }
    });

  println!("");
}

fn main() {
  let ls_args: Vec<String> = args().skip(1).collect();
  let ls_options = ls_args.iter().find(| a | a.starts_with('-'));
  let ls_dir = ls_args.iter().find(|a| !a.starts_with('-'));

  let options = match ls_options {
    Some(option) => { option.chars().collect() }
    None => { vec![] }
  };

  let dir = match ls_dir {
      Some(dir) => { dir }
      None => { "." }
  };

  let ignore_hidden = !options.contains(&'a');
  let dir_entries = get_entries(&dir, ignore_hidden);
  let _ = if options.contains(&'l') {
    print_list(dir_entries)
  } else {
    print_names(dir_entries)
  };
}
