slint::slint! {
    import {
        Button,
        LineEdit,
        VerticalBox
    } from "std-widgets.slint";

    export component TaxCalculator {
        in-out property <int> price: 100;
        in property <int> tax: 0;
        in property <int> total: 0;
        callback calculate <=> calculate-button.clicked;

        VerticalBox {
            Text {
                text: "Tax calculator";
                horizontal-alignment: TextHorizontalAlignment.center;
                vertical-alignment: TextVerticalAlignment.center;
            }
            GridLayout {
                spacing: 5px;
                Row {
                    Text {
                        text: "Price";
                        vertical-alignment: TextVerticalAlignment.center;
                    }
                    LineEdit {
                        text: price;
                        edited => {
                            price = self.text.to-float();
                        }
                    }
                }
                Row {
                    Text {
                        text: "Tax";
                        vertical-alignment: TextVerticalAlignment.center;
                    }
                    LineEdit {
                        text: tax;
                        read-only: true;
                    }
                }
                Row {
                    Text {
                        text: "Total";
                        vertical-alignment: TextVerticalAlignment.center;
                    }
                    LineEdit {
                        text: total;
                        read-only: true;
                    }
                }
            }
            calculate-button := Button {
                text: "Calculate";
                primary: true;
            }
        }
    }
}

fn main() {
    let calculator = TaxCalculator::new().unwrap();
    let weak = calculator.as_weak();
    calculator.on_calculate(move || {
        let calculator = weak.upgrade().unwrap();
        let price = calculator.get_price();
        println!("price: {}", price);
        let tax = price / 10;
        let total = price + tax;
        calculator.set_tax(tax);
        calculator.set_total(total);
    });
    calculator.run().unwrap();
}
