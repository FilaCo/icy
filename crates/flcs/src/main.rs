use flcs::prelude::*;

fn main() -> anyhow::Result<()> {
    iced::daemon(Flcs::title, Flcs::update, Flcs::view).run_with(Flcs::new)?;
    Ok(())
}
