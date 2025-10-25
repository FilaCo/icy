use filaco_shell::Shell;

#[tokio::main(flavor = "current_thread")]
async fn main() -> iced::Result {
    iced::daemon(Shell::title, Shell::update, Shell::view)
        .subscription(Shell::subscription)
        .run_with(Shell::new)
}
