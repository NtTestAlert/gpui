use gpui::{
    AnyElement, AppContext, Context, Entity, IntoElement, ParentElement, Render, TestAppContext,
    Window, div,
};
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
    let mut element = div()
        .on_children_prepainted(move |_, _, _| {
            let _ = &retained;
        })
        .into_any_element();
    assert_eq!(Rc::strong_count(&capture), 2);
    assert!(element.downcast_mut::<gpui::Div>().is_some());
    drop(element);
    assert_eq!(Rc::strong_count(&capture), 1);
}

#[gpui::test]
fn prebuilt_child_drops_after_its_first_frame(cx: &mut TestAppContext) {
    struct Once(Option<AnyElement>);
    impl Render for Once {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div().children(self.0.take())
        }
    }

    let prepaint_count = Rc::new(Cell::new(0));
    let captured = prepaint_count.clone();
    let child = div()
        .on_children_prepainted(move |_, _, _| captured.set(captured.get() + 1))
        .into_any_element();
    cx.add_window(|_, _| Once(Some(child)));
    cx.run_until_parked();
    assert_eq!(prepaint_count.get(), 1);
    assert_eq!(Rc::strong_count(&prepaint_count), 1);
}

#[gpui::test]
fn reads_follow_mounts_and_surviving_windows(cx: &mut TestAppContext) {
    struct Reader {
        entity: Option<Entity<()>>,
        renders: Rc<Cell<usize>>,
    }
    impl Render for Reader {
        fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            if let Some(entity) = &self.entity {
                entity.read(cx);
            }
            self.renders.set(self.renders.get() + 1);
            div()
        }
    }

    let shared = cx.new(|_| ());
    let first_renders = Rc::new(Cell::new(0));
    let second_renders = Rc::new(Cell::new(0));
    let mut reader = |renders: &Rc<Cell<usize>>| {
        cx.add_window(|_, _| Reader {
            entity: Some(shared.clone()),
            renders: renders.clone(),
        })
    };
    let first = reader(&first_renders);
    let second = reader(&second_renders);
    cx.run_until_parked();
    let both_mounted = cx.read(|app| app.tracked_render_entity_count());

    first
        .update(cx, |view, _, cx| {
            view.entity = None;
            cx.notify();
        })
        .unwrap();
    cx.run_until_parked();
    let first_before = first_renders.get();
    let second_before = second_renders.get();
    shared.update(cx, |_, cx| cx.notify());
    cx.run_until_parked();
    assert_eq!(first_renders.get(), first_before);
    assert!(second_renders.get() > second_before);

    second
        .update(cx, |view, _, cx| {
            view.entity = None;
            cx.notify();
        })
        .unwrap();
    cx.run_until_parked();
    // The entity is still alive, but its last reader left.
    assert_eq!(
        cx.read(|app| app.tracked_render_entity_count()),
        both_mounted - 1
    );

    let observed = Rc::new(Cell::new(0));
    let notifications = observed.clone();
    let _subscription = cx.update(|app| {
        app.observe(&shared, move |_, _| {
            notifications.set(notifications.get() + 1)
        })
    });
    let first_before = first_renders.get();
    let second_before = second_renders.get();
    shared.update(cx, |_, cx| cx.notify());
    cx.run_until_parked();
    assert_eq!(observed.get(), 1);
    assert_eq!(first_renders.get(), first_before);
    assert_eq!(second_renders.get(), second_before);
    assert_eq!(
        cx.read(|app| app.tracked_render_entity_count()),
        both_mounted - 1
    );

    for window in [first, second] {
        window
            .update(cx, |view, _, cx| {
                view.entity = Some(shared.clone());
                cx.notify();
            })
            .unwrap();
    }
    cx.run_until_parked();
    assert_eq!(
        cx.read(|app| app.tracked_render_entity_count()),
        both_mounted
    );
    let first_before = first_renders.get();
    let second_before = second_renders.get();
    shared.update(cx, |_, cx| cx.notify());
    cx.run_until_parked();
    assert!(first_renders.get() > first_before);
    assert!(second_renders.get() > second_before);

    first
        .update(cx, |_, window, _| window.remove_window())
        .unwrap();
    cx.run_until_parked();
    let second_before = second_renders.get();
    shared.update(cx, |_, cx| cx.notify());
    cx.run_until_parked();
    assert!(second_renders.get() > second_before);
    second
        .update(cx, |_, window, _| window.remove_window())
        .unwrap();
    cx.run_until_parked();
    assert_eq!(cx.read(|app| app.tracked_render_entity_count()), 0);
}
