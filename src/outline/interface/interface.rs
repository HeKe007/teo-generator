use std::cmp::Ordering;
use itertools::Itertools;
use teo_parser::r#type::Type;
use crate::outline::interface::Field;

pub(crate) struct Interface {
    pub(in super::super) title: String,
    pub(in super::super) desc: String,
    pub(in super::super) path: Vec<String>,
    pub(in super::super) name: String,
    pub(in super::super) generic_names: Vec<String>,
    pub(in super::super) extends: Vec<Type>,
    pub(in super::super) fields: Vec<Field>,
    pub(in super::super) synthesized: Option<(String, Option<String>)>,
    pub(in super::super) model_name: Option<String>,
}

impl Interface {

    pub(crate) fn title(&self) -> &str {
        self.title.as_str()
    }

    pub(crate) fn desc(&self) -> &str {
        self.desc.as_str()
    }

    pub(crate) fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(crate) fn generic_names(&self) -> &Vec<String> {
        &self.generic_names
    }

    pub(crate) fn extends(&self) -> &Vec<Type> {
        &self.extends
    }

    pub(crate) fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub(crate) fn fields_optional_at_last(&self) -> Vec<&Field> {
        self.fields.iter().sorted_by(|a, _b| if a.r#type().is_optional() { Ordering::Greater } else { Ordering::Less }).collect()
    }

    pub(crate) fn synthesized(&self) -> &Option<(String, Option<String>)> {
        &self.synthesized
    }

    pub(crate) fn generics_declaration(&self) -> String {
        if self.generic_names().is_empty() {
            "".to_owned()
        } else {
            "<".to_owned() + &self.joined_generic_names() + ">"
        }
    }

    pub(crate) fn joined_generic_names(&self) -> String {
        self.generic_names().join(", ")
    }

    pub(crate) fn is_output_result(&self) -> bool {
        if let Some(synthesized) = self.synthesized() {
            synthesized.0.as_str() == "Result"
        } else {
            false
        }
    }

    pub(crate) fn model_name(&self) -> &str {
        if let Some(model_name) = &self.model_name {
            model_name.as_str()
        } else {
            ""
        }
    }
}