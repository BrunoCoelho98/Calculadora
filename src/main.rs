slint::include_modules!();
use eval::eval;
use slint::SharedString;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();

    ui.on_calculate(move |string: slint::SharedString|{
        let ui = ui_handle.unwrap();
        
        let result: Result<eval::Value, eval::Error> = match eval(string.as_str()) {
            Ok(result) => Ok(result),
            Err(error) => Err(error.into()),
        };

        match result {
            Ok(value) => {
                if value.to_string() != "null" {
                    ui.set_result(SharedString::from(value.to_string()));
                } else {
                    ui.set_result("".into());
                }},
            Err(_error) => ui.set_result("".into()),
        }
    });

    ui.run()
}
