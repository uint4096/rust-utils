use crate::file::permission::Permission;
use crate::utils::time;
use std::{
    fs::Metadata,
    io,
    os::unix::prelude::{MetadataExt, PermissionsExt},
    time::SystemTime,
};
use users::{get_group_by_gid, get_user_by_uid};

pub struct LSRow {
    is_file: bool,
    permissions: Permission,
    owner: String,
    group: String,
    last_modified: SystemTime,
    size: u64,
    links: u64,
    name: String,
}

impl LSRow {
    pub fn new(name: &str, metadata: Metadata) -> io::Result<LSRow> {
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

        Ok(LSRow {
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

    pub fn format_rows(mut rows: Vec<LSRow>) -> Vec<String> {
        let format_with_spaces = |max_len: usize, elem: &String| {
            let num_spaces = max_len - elem.len();
            let spaces = (0..num_spaces).fold(String::from(""), |mut s, _| {
                s.push(' ');
                s
            });
            let mut final_str = String::from(elem);
            final_str.push_str(&spaces);
            final_str
        };

        let (user_max_len, group_max_len, size_max_len) =
            rows.iter().fold((0, 0, 0), |mut tup, elem| {
                if elem.owner.len() > tup.0 {
                    tup.0 = elem.owner.len();
                }
                if elem.group.len() > tup.1 {
                    tup.1 = elem.group.len();
                }
                if elem.size.to_string().len() > tup.2 {
                    tup.2 = elem.size.to_string().len();
                }
                tup
            });

        rows.sort_by(|a, b| b.is_file.cmp(&a.is_file));
        rows.iter()
            .map(|row| {
                format!(
                    "{}{} {} {} {} {} {} {}",
                    if row.is_file { "-" } else { "d" },
                    row.permissions.get_str_permissions(),
                    row.links,
                    format_with_spaces(user_max_len, &row.owner),
                    format_with_spaces(group_max_len, &row.group),
                    format_with_spaces(size_max_len, &row.size.to_string()),
                    time::format_system_time(row.last_modified),
                    if row.is_file {
                        format!("{}", row.name)
                    } else {
                        format!("\x1b[31;1m{}\x1b[0m", row.name)
                    }
                )
            })
            .collect()
    }
}
