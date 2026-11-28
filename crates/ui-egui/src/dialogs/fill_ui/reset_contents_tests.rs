//! #14642: Fill opens (and Alt+Reset returns) on the fill-from-surroundings entry, as the
//! reference application does. Remembered choices after OK still win.

use super::*;
use serde_json::json;

fn app() -> Box<PhotocraftApp> {
    let mut app = PhotocraftApp::new(photocraft_engine::Session::new(), Default::default());
    app.run("file.new", json!({"width": 40, "height": 30, "background": "transparent"})).unwrap();
    app.run("tools.setColors", json!({"foreground": "#ff0000", "background": "#0000ff"})).unwrap();
    app
}

#[test]
fn the_contents_row_shows_content_aware_when_nothing_is_remembered() {
    let app = app();
    let rows = rows(&app, [1.0, 0.0, 0.0], None, None);
    let contents = rows.iter().find(|r| r.key == "contents").expect("the Contents row");
    match &contents.kind {
        ParamKind::Choice { default, .. } => assert_eq!(default.as_deref(), Some("contentAware")),
        other => panic!("Contents is a choice, not {other:?}"),
    }
}

#[test]
fn reset_returns_to_content_aware_with_the_measured_options() {
    let f = defaults([1.0, 0.0, 0.0], None, None, None);
    assert_eq!(f.text("contents"), Some("contentAware"), "Alt+Reset sets the fill-from-surroundings entry");
    assert_eq!(f.bool("colorAdaptation"), Some(true));
    assert_eq!(f.text("mode"), Some("normal"));
    assert_eq!(f.number("opacity"), Some(100.0));
    assert_eq!(f.bool("preserveTransparency"), Some(false));
}

#[test]
fn a_forced_preserve_transparency_keeps_its_state_on_reset() {
    let f = defaults([1.0, 0.0, 0.0], None, None, Some(true));
    assert_eq!(f.bool("preserveTransparency"), Some(true));
}
