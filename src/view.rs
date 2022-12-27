use druid::{widget::{Button, Label, Flex, Align}, Widget, WindowDesc, WindowState};

use crate::data::*;

pub fn preview_ui() -> impl Widget<AppState> {
  Label::new("Hello")
}

pub fn build_ui() -> impl Widget<AppState> {
  let new_window = Button::new("Enter").on_click(|_ctx, _data, _env| {
    let new_win = WindowDesc::new(preview_ui()).set_window_state(WindowState::Maximized).show_titlebar(false);
    _ctx.new_window(new_win);
  });

  let layout = Flex::column().with_child(new_window);

  Align::centered(layout)
}