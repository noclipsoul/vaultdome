use druid::widget::{Button, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use reqwest; // Add this import for HTTP requests
use serde_json::json; // Import for creating JSON payload


#[derive(Clone, Data, Lens)]
struct AppState {
    username: String,
    password: String,
}

fn build_ui() -> impl Widget<AppState> {
    let username_input = TextBox::new().with_placeholder("Username").lens(AppState::username);
    let password_input = TextBox::new().with_placeholder("Password").lens(AppState::password);

    let login_button = Button::new("Login").on_click(|_, data: &mut AppState, _| {
        let username = data.username.clone();
        let password = data.password.clone();

        // Create a new async task to handle the login request
        let request = async move {
            // Create JSON payload
            let login_request = json!({
                "username": username,
                "password": password,
            });

            // Send the login request
            match reqwest::Client::new()
                .post("http://127.0.0.1:15001/login") // Update the URL to match your backend
                .json(&login_request)
                .send()
                .await {
                Ok(response) => {
                    // Handle the response here
                    if response.status().is_success() {
                        let login_response: serde_json::Value = response.json().await.unwrap();
                        println!("{:?}", login_response); // Print the response
                    } else {
                        println!("Login failed: {:?}", response.status());
                    }
                },
                Err(err) => {
                    println!("Error making request: {:?}", err);
                }
            }
        };

        // Spawn the async task
        tokio::spawn(request);
    });

    Flex::column()
        .with_child(username_input)
        .with_spacer(8.0)
        .with_child(password_input)
        .with_spacer(8.0)
        .with_child(login_button)
        .padding(10.0)
}

// In your gui.rs file

// Change run_gui to be async
pub async fn run_gui() {
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("login-window-title").with_placeholder("Login"));

    let initial_state = AppState {
        username: "".into(),
        password: "".into(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}


