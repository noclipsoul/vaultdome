use druid::widget::{Button, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppState {
    username: String,
    password: String,
}

fn build_ui() -> impl Widget<AppState> {
    let username_input = TextBox::new().with_placeholder("Username").lens(AppState::username);
    let password_input = TextBox::new().with_placeholder("Password").lens(AppState::password);

    let login_button = Button::new("Login").on_click(|_, _, _| {
        // Currently does nothing
    });

    Flex::column()
        .with_child(username_input)
        .with_spacer(8.0)
        .with_child(password_input)
        .with_spacer(8.0)
        .with_child(login_button)
        .padding(10.0)
}

pub fn run_gui() {
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("login-window-title").with_placeholder("Login"));

    let initial_state = AppState {
        username: "".into(),
        password: "".into(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console() // Updated method
        .launch(initial_state)
        .expect("Failed to launch application");
}
