
pub struct PermissionTypes {
  read: bool,
  write: bool,
  execute: bool
}

impl PermissionTypes {
  pub fn create(permission_bits: u32) -> PermissionTypes {
    PermissionTypes {
      read: permission_bits >> 2 == 1,
      write: (permission_bits >> 1) & 0b01 == 1,
      execute: permission_bits & 0b001 == 1, 
    }
  }

  pub fn format(&self) -> String {
    let find_perm = | prop: bool, char: char | if prop { char } else { '-' };
    let mut permission = String::from("");
    permission.push(find_perm(self.read, 'r'));
    permission.push(find_perm(self.write, 'w'));
    permission.push(find_perm(self.execute, 'x'));
    permission
  }
}
