use std::str::FromStr;

use clap;
use cursive::views;
use serde_json::value::Value;

use fields;
use fields::{FieldErrors, WidgetManager};

/// Convienient wrapper around `Field<CheckboxManager, bool>`.
pub struct Checkbox;

impl Checkbox {
    /// Creates a new `Field<CheckboxManager, bool>`.
    pub fn new<IS: Into<String>>(label: IS) -> fields::Field<CheckboxManager, bool> {
        fields::Field::new(label, CheckboxManager, false)
    }
}

#[derive(Clone)]
pub struct CheckboxManager;

impl fields::WidgetManager for CheckboxManager {
    fn build_widget(&self, label: &str, help: &str, initial: &str) -> views::ViewBox {
        let checkbox = self.build_value_view(&initial);
        fields::label_with_help_layout(checkbox, &label, &help)
    }
    fn get_value(&self, view_box: &views::ViewBox) -> String {
        let view_box = fields::value_view_from_layout(view_box);
        let checkbox: &views::Checkbox = (**view_box).as_any().downcast_ref().unwrap();
        let value = checkbox.is_checked();
        format!("{}", value)
    }
    fn build_value_view(&self, value: &str) -> views::ViewBox {
        let value = FromStr::from_str(value).unwrap();
        let mut checkbox = views::Checkbox::new();
        checkbox.set_checked(value);
        views::ViewBox::new(Box::new(checkbox))
    }
}

impl fields::FormField for fields::Field<CheckboxManager, bool> {
    fn get_widget_manager(&self) -> &WidgetManager {
        &self.widget_manager
    }
    fn build_widget(&self) -> views::ViewBox {
        let initial = format!("{}", self.initial);
        let view = self.widget_manager.build_value_view(&initial);
        fields::label_with_help_layout(view, &self.label, &self.help)
    }
    fn get_label(&self) -> &str {
        &self.label
    }

    /// Gets help of the field
    fn get_help(&self) -> &str {
        self.help.as_ref()
    }

    fn validate(&self, data: &str) -> Result<Value, FieldErrors> {
        let result = FromStr::from_str(data)
            .map(|v| Value::Bool(v))
            .map_err(|_| {
                let mut errors = FieldErrors::new();
                let error = "Value can't be converterd to bool".to_string();
                errors.push(error);
                errors
            });
        result
    }

    fn clap_arg(&self) -> clap::Arg {
        clap::Arg::with_name(&self.label)
            .help(&self.help)
            .long(&self.label)
    }

    fn clap_args2str(&self, args: &clap::ArgMatches) -> String {
        let v = if args.is_present(&self.label) {
            "true"
        } else {
            "false"
        };
        v.to_string()
    }

    fn is_required(&self) -> bool {
        self.is_required()
    }
}

impl<W: WidgetManager> fields::Field<W, bool> {
    /// Sets initial `value` of `field`.
    pub fn initial(mut self, value: bool) -> Self {
        self.initial = value;
        self
    }
}
