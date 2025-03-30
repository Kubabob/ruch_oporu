use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct InputProps {
    pub input_type: String,
    pub label: String,
    pub placeholder: String,
    pub value: UseStateHandle<String>
}


#[function_component(InputField)]
pub fn input_field(props: &InputProps) -> Html {
    let input_type = props.input_type.clone();
    let label1 = props.label.clone();
    let label2 = props.label.clone();
    let label3 = props.label.clone();
    let placeholder = props.placeholder.clone();
    let value = props.value.clone();

    html!(
        <>
            <label for={label1}>{label2}</label>
            <input type={input_type} name={label3} placeholder={placeholder} value={(*value).clone()} required={true}/>
        </>
    )
}


#[derive(Properties, PartialEq, Debug)]
pub struct TextAreaProps {
    pub label: String,
    pub placeholder: String,
    pub height: i32,
    pub value: UseStateHandle<String>
}

#[function_component(TextAreaField)]
pub fn text_area_field(props: &TextAreaProps) -> Html {
    let label1 = props.label.clone();
    let label2 = props.label.clone();
    let label3 = props.label.clone();
    let placeholder = props.placeholder.clone();
    let height = props.height.to_string();
    let value = props.value.clone();

    html!(
        <>
            <label for={label1}>{label2}</label>
            <textarea id={label3} placeholder={placeholder} cols={height} value={(*value).clone()} required={true}>
            </textarea>
        </>
    )
}


#[derive(Properties, PartialEq, Debug)]
pub struct SubmitButtonProps {
    pub label: String
}

#[function_component(SubmitButton)]
pub fn submit_button(props: &SubmitButtonProps) -> Html {
    let label = props.label.clone();

    html!(
        <>
            <input class="submit-button" type="submit" value={label}/>
        </>
    )
}

#[derive(Properties, PartialEq, Debug)]
pub struct RadioInputProps {
    pub label: String,
    pub name: String,
    pub value: String,
    pub checked: bool
}

#[function_component(RadioInput)]
pub fn radio_input(props: &RadioInputProps) -> Html {
    let label1 = props.label.clone();
    let label2 = props.label.clone();
    let label3 = props.label.clone();
    let label4 = props.label.clone();
    let name = props.name.clone();
    let checked = props.checked.clone();

    html!(
        <>
        <input type="radio" id={label3} name={name} value={label4} checked={checked}/>
        <label for={label1}>{label2}</label>
        </>
    )
}

#[derive(Properties, PartialEq, Debug)]
pub struct CheckboxInputProps {
    pub label: String,
    pub name: String,
    pub value: String,
    pub checked: bool
}

#[function_component(CheckboxInput)]
pub fn checkbox_input(props: &CheckboxInputProps) -> Html {
    let label1 = props.label.clone();
    let label2 = props.label.clone();
    let label3 = props.label.clone();
    let label4 = props.label.clone();
    let name = props.name.clone();
    let checked = props.checked.clone();

    html!(
        <div>
            <input type="checkbox" id={label3} name={name} value={label4} checked={checked}/>
            <label for={label1}>{label2}</label>
        </div>
    )
}


#[function_component(ContactBox)]
pub fn contact_box() -> Html {
    let email = use_state(|| String::new());
    let message = use_state(|| String::new());
    
    html!{
        <div class="form-box-wrapper">
            <div class="contaxt-text-wrapper">
                <h2 class="zainteresowanie">{"Zainteresowały Cię nasze działania? A może masz pytania?"}</h2>
                <h2 class="zostaw-wiadomosc">{"Zostaw nam wiadomość!"}</h2>
            </div>

            <form class="contact-form">
                <InputField input_type="email" label="Email" placeholder="Wpisz" value={email} />
                <TextAreaField label="Wiadomość" placeholder="Wpisz" height=2 value={message} />
                <SubmitButton label="Wyślij" />
            </form>
        
        </div>
    }
}