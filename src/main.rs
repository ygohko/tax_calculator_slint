slint::slint! {
    import {
        AboutSlint,
        Button,
        HorizontalBox,
        LineEdit,
        StandardButton,
        VerticalBox
    } from "std-widgets.slint";

    export component TaxCalculator inherits Window {
        in property <string> price <=> price-line-edit.text;
        in property <string> tax;
        in property <string> total;
        callback calculate <=> calculate-button.clicked;
        callback show_about <=> about-button.clicked;
        title: "Tax calculator";
        preferred-width: 400px;
        preferred-height: 300px;

        VerticalBox {
            Text {
                text: "Tax calculator";
                font-size: 20px;
                font-weight: 600;
                horizontal-alignment: TextHorizontalAlignment.center;
                vertical-alignment: TextVerticalAlignment.center;
            }
            Rectangle {
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
            Rectangle {
            }
            HorizontalLayout {
                alignment: LayoutAlignment.center;
                calculate-button := Button {
                    text: "Calculate";
                    primary: true;
                    width: parent.width * 60%;
                }
            }
            Rectangle {
            }
            HorizontalLayout {
                alignment: LayoutAlignment.end;
                about-button := Button {
                    text: "About";
                }
            }
        }
    }

    export component AboutDialog inherits Dialog {
        title: "About";
        VerticalBox {
            Text {
                text: "tax_calculator_slint";
                font-size: 20px;
                font-weight: 600;
                horizontal-alignment: TextHorizontalAlignment.center;
            }
            Text {
                text: "0.1.0";
                horizontal-alignment: TextHorizontalAlignment.center;
            }
            Text {
                text: "(c) Yasuaki Gohko";
                horizontal-alignment: TextHorizontalAlignment.center;
            }
            HorizontalLayout {
                alignment: LayoutAlignment.center;
                max-height: 90px;
                min-height: 90px;
                AboutSlint {
                }
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
    calculator.on_show_about(move || {
        let dialog = AboutDialog::new().unwrap();
        dialog.show().unwrap();
    });
    calculator.set_price("0".into());
    calculator.set_tax("0".into());
    calculator.set_total("0".into());
    calculator.run().unwrap();
}
