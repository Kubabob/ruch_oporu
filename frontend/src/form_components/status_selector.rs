use yew::prelude::*;
use super::questions_section::Status;

#[derive(Properties, PartialEq)]
pub struct StatusSelectorProps {
    pub on_status_change: Callback<Status>,
    pub current_status: Status,
}

#[function_component(StatusSelector)]
pub fn status_selector(props: &StatusSelectorProps) -> Html {
    let on_change = |value: Status| {
        let cb = props.on_status_change.clone();
        Callback::from(move |_| cb.emit(value.clone()))
    };

    html! {
        <div class="status-selector">
            <label class={classes!(if props.current_status == Status::LGBT { "active" } else { "" })}>
                <input
                    type="radio"
                    name="status"
                    checked={props.current_status == Status::LGBT}
                    onchange={on_change(Status::LGBT)}
                />
                {"LGBT"}
            </label>
            
            <label class={classes!(if props.current_status == Status::Ally { "active" } else { "" })}>
                <input
                    type="radio"
                    name="status"
                    checked={props.current_status == Status::Ally}
                    onchange={on_change(Status::Ally)}
                />
                {"Ally"}
            </label>
            
            <label class={classes!(if props.current_status == Status::Innx { "active" } else { "" })}>
                <input
                    type="radio"
                    name="status"
                    checked={props.current_status == Status::Innx}
                    onchange={on_change(Status::Innx)}
                />
                {"Innx"}
            </label>
        </div>
    }
}