use crate::app::App;

mod app;

fn main() {
    let mut app = App::create();
    app.run();
}
