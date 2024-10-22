#![allow(unused)]

use std::{fs::read_to_string, sync::Arc};
use gtk::prelude::*;
use gtk::*;
use relm::Widget;
use relm_derive::{widget, Msg};
use toml::Value;
use crate::utils::{traits::*, FontType, PersonalizationConfig, PERSONALIZATION_CONFIG};

pub struct HeadingModel {
    label: &'static str,
    font_type: FontType,
}

#[widget]
impl Widget for Heading {
    fn model(data: (
        &'static str,
        Arc<PersonalizationConfig>,
    )) -> HeadingModel {
        HeadingModel {
            label: data.0,
            font_type: data.1.font_type,
        }
    }

    fn update(&mut self, _: ()) {}

    view! {
        gtk::Label {
            text: &match self.model.font_type {
                FontType::Dot => self.model.label.to_uppercase(),
                FontType::Serif => self.model.label.to_owned(),
            },
            font_family: self.model.font_type.to_font_family(),
            font_size: 24,
            margin_bottom: 10,
        },
    }
}