use gpui::{AppContext, IntoElement, TestAppContext, div};
use std::{cell::Cell, rc::Rc};

#[gpui::test]
fn notifying_unrendered_entity_keeps_index_empty(cx: &mut TestAppContext) {
    let entity = cx.new(|_| ());
    let notifications = Rc::new(Cell::new(0));
    let observed = notifications.clone();
    let _subscription =
        cx.update(|app| app.observe(&entity, move |_, _| observed.set(observed.get() + 1)));
    entity.update(cx, |_, cx| cx.notify());
    cx.run_until_parked();
    assert_eq!(notifications.get(), 1);
    assert_eq!(cx.read(|app| app.tracked_render_entity_count()), 0);
}

#[test]
fn dropping_element_outside_draw_releases_its_capture() {
    let capture = Rc::new(());
    let retained = capture.clone();
    let element = div()
        .on_children_prepainted(move |_, _, _| {
            let _ = &retained;
        })
        .into_any_element();
    assert_eq!(Rc::strong_count(&capture), 2);
    drop(element);
    assert_eq!(Rc::strong_count(&capture), 1);
}
