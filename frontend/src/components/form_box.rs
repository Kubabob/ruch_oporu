use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct InputProps {
    pub input_type: String,
    pub label: String,
    pub placeholder: String
}


#[function_component(InputField)]
pub fn input_field(props: &InputProps) -> Html {
    let input_type = props.input_type.clone();
    let label1 = props.label.clone();
    let label2 = props.label.clone();
    let label3 = props.label.clone();
    let placeholder = props.placeholder.clone();

    html!(
        <>
            <label for={label1}>{label2}</label>
            <input type={input_type} name={label3} placeholder={placeholder} required={true}/>
        </>
    )
}


#[derive(Properties, PartialEq, Debug)]
pub struct TextAreaProps {
    pub label: String,
    pub placeholder: String,
    pub height: i32
}

#[function_component(TextAreaField)]
pub fn text_area_field(props: &TextAreaProps) -> Html {
    let label1 = props.label.clone();
    let label2 = props.label.clone();
    let label3 = props.label.clone();
    let placeholder = props.placeholder.clone();
    let height = props.height.to_string();

    html!(
        <>
            <label for={label1}>{label2}</label>
            <textarea name={label3} placeholder={placeholder} cols={height} required={true}>
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


#[function_component(ContactBox)]
pub fn contact_box() -> Html {
    html!{
        <div class="contact-box-wrapper">
            <div class="contaxt-text-wrapper">
                <h2 class="zainteresowanie">{"Zainteresowały Cię nasze działania? A może masz pytania?"}</h2>
                <h2 class="zostaw-wiadomosc">{"Zostaw nam wiadomość!"}</h2>
            </div>

            <form class="contact-form">
                <InputField input_type="email" label="Email" placeholder="Wpisz" />
                <TextAreaField label="Wiadomość" placeholder="Wpisz" height=2 />
                <SubmitButton label="Wyślij" />
            </form>
        
        </div>
    }
}