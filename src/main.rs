use std::fs::read_to_string;

use anathema::prelude::*;

fn main() {
    let template = read_to_string("template.aml").unwrap();

    let mut doc = Document::new(template);

    let mut backend = TuiBackend::builder()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);
    runtime.finish().unwrap().run();
}
