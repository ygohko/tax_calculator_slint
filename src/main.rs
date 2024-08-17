slint::slint! {
    import {
        Button,
        LineEdit,
        VerticalBox
    } from "std-widgets.slint";

    export component App {
        in-out property <int> price: 100;
        in property <int> tax: 0;
        in property <int> total: 0;
        callback clicked <=> calculateButton.clicked;

        VerticalBox {
            Text {
                text: "Tax calculator";
            }
            LineEdit {
                text: price;
                edited => {
                    price = self.text.to-float();
                }
            }
            LineEdit {
                text: tax;
            }
            LineEdit {
                text: total;
            }
            calculateButton := Button {
                text: "Calculate";
            }
        }
    }
}

fn main() {
    let app = App::new().unwrap();
    let weak = app.as_weak();
    app.on_clicked(move || {
        let app = weak.upgrade().unwrap();
        let price = app.get_price();
        println!("price: {}", price);
        let tax = price / 10;
        let total = price + tax;
        app.set_tax(tax);
        app.set_total(total);
    });
    app.run().unwrap();
}
