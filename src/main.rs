slint::slint! {
    import {
        Button,
        LineEdit,
        VerticalBox
    } from "std-widgets.slint";

    export component TaxCalculator {
        in property <string> price <=> price-line-edit.text;
        in property <string> tax;
        in property <string> total;
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
                    price-line-edit := LineEdit {
                        input-type: number;
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
        let price = match calculator.get_price().parse::<i32>() {
            Ok(price) => price,
            Err(_) => 0,
        };
        let tax = price / 10;
        let total = price + tax;
        calculator.set_tax(tax.to_string().into());
        calculator.set_total(total.to_string().into());
    });
    calculator.set_price("0".into());
    calculator.set_tax("0".into());
    calculator.set_total("0".into());
    calculator.run().unwrap();
}
