#![allow(unused)]

use gtk::prelude::*;
use gtk::*;
use relm::Widget;
use relm_derive::{widget, Msg};
use crate::utils::traits::*;

pub struct HeaderModel(&'static str);

#[widget]
impl Widget for Header {
    fn model(label: &'static str) -> HeaderModel {
        HeaderModel(label)
    }

    fn update(&mut self, _: ()) {}

    view! {
        gtk::HeaderBar {
            custom_title: view! {
                gtk::Label {
                    text: self.model.0,
                    font_family: "NDot",
                    font_size: 18,
                    margin_top: 5,
                }
            },
            show_close_button: true,
        }
    }
}