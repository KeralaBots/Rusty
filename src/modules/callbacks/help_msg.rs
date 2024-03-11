use lazy_static::lazy_static;

use crate::utils::escape_markdown;

lazy_static!{
    pub static ref ADMINS: String = format!(
    "*ADMIN*\n\n{}\n\n*Admin commands*{}", 
    escape_markdown("Make it easy to promote and demote users with the admin module!"),
    "\n\nCommands"
);
}
