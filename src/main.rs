slint::slint! {
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App {
        VerticalBox {
            Text {
                text: "Hello, World";
            }
        }
    }
}

fn main() {
    let app = App::new().unwrap();
    app.run().unwrap();
}
