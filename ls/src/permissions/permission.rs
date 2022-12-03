use crate::permissions::types::PermissionTypes;

pub struct Permission {
    user: String,
    group: String,
    others: String,
}

impl Permission {
    // Look up man -s 7 inode
    pub fn get_permission(st_mode: u32) -> Permission {
        /*
         * st_mode
         * The last (least significant) 9 bits are permission bits. The bits before that represent the file type.
         * To extract the last 9 bits we need to do a bitwise and with a mask of 111111111.
         * This octal representation for that number is 0o0777.
         */
        let permission_mask = 0o0777;
        let permission_bits = st_mode & permission_mask;

        /*
         * The first 3 bits are user permissions. The next 3, group permissions, and the last 3 others.
         * We can get the first 3 bits by right shifting the bits 6 places.
         * We can get the middle 3 by right shifting the bits by 3 places and doing a bitwise & with a mask of 000111.
         * We can get the last 3 bits by simply doing a bitwise & with a mask of 000111,
         */

        let user_permission = PermissionTypes::create(permission_bits >> 6);
        let group_permission = PermissionTypes::create((permission_bits >> 3) & 0b111);
        let others_permission = PermissionTypes::create(permission_bits & 0b111);

        Permission {
            user: user_permission.format(),
            group: group_permission.format(),
            others: others_permission.format(),
        }
    }

    pub fn get_str_permissions(&self) -> String {
        let mut init_str = String::from(&self.user);
        init_str.push_str(&self.group);
        init_str.push_str(&self.others);
        init_str
    }
}
