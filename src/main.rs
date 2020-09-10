use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;
use std::time::SystemTime;

use chrono::prelude::*;

fn main() {
    let mut siv = cursive::default();

    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .fixed_width(20);
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Morning meeting")
        .with_name("main_dialog"),
    );

    siv.refresh();
    while siv.is_running() {
        // let date = Local::now();
        // siv.call_on_name("main_dialog", |view: &mut Dialog| {
        //     view.set_title(format!("Time: {}", date.format("%H:%M:%S")));
        // });
        siv.step();
    }
}

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        let formatted_name = format!("[ ] - {}", name);
        let mut num_attendees = 0usize;
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(&formatted_name);
            num_attendees = view.len();
        });
        s.pop_layer();

        s.call_on_name("main_dialog", |view: &mut Dialog| {
            view.set_title(format!("Morning meeting - {} attendees", num_attendees));
        });
    }
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("name")
                .fixed_width(10),
        )
        .title("Enter a new name")
        .button("Ok", |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }

    s.call_on_name("main_dialog", |view: &mut Dialog| {
        view.set_title(format!("Morning meeting - {} attendees", select.len()));
    });
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.screen_mut();
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
            let mut new_string = String::from(name);
            let start = name.find("X");
            if start.is_none() {
                new_string = new_string.replace("[ ]", "[X]");
            } else {
                new_string = new_string.replace("[X]", "[ ]");
            }
            select.insert_item_str(focus, &new_string);
            select.set_selection(focus);
            ()
        }
    }
}
