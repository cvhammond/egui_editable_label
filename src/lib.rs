//! # egui_editable_label
//!
//! A simple widget for egui that allows the user to edit a label in place.
//!
//! ## Usage
//! ```no_run
//! ui.add(EditableLabel::new(&mut self.label, Id::new("editable_label")));
//! ```

use egui::*;

pub struct EditableLabel<'a> {
    id: Id,
    label: &'a mut dyn TextBuffer,
}

impl<'a> EditableLabel<'a> {
    pub fn new(label: &'a mut impl TextBuffer, id: Id) -> Self {
        Self { id, label }
    }
}

impl<'a> Widget for EditableLabel<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let in_edit_mode = match ui.data_mut(|id_map| id_map.get_temp::<bool>(self.id)) {
            Some(in_edit_mode) => in_edit_mode,
            None => false,
        };
        if in_edit_mode {
            let response = ui.add(
                TextEdit::singleline(self.label)
                    .horizontal_align(ui.layout().horizontal_align())
                    .vertical_align(ui.layout().vertical_align())
                    .margin(ui.spacing().button_padding)
                    .min_size(ui.spacing().interact_size)
                    .desired_width(ui.spacing().interact_size.x),
            );
            if response.lost_focus() {
                ui.data_mut(|id_map| id_map.insert_temp(self.id, false));
            }
            response.request_focus();
            response
        } else {
            let response = ui.add(Label::new(self.label.as_str()));
            if response.clicked() {
                ui.data_mut(|id_map| id_map.insert_temp(self.id, true));
            }
            response
        }
    }
}
