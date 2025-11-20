use fs_shell::{prelude::Shell, shell::ShellError};

fn main() -> Result<(), ShellError> {
    Shell::default().run()
}
