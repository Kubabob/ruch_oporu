/*use yew::prelude::*;
use web_sys::{HtmlInputElement, FileList};

pub fn survey_file_upload() -> Html {
    let file_input_ref = use_node_ref();

    let on_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();

            }
        }
    })
}
*/