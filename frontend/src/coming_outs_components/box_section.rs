use yew::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, Element, HtmlImageElement};
use wasm_bindgen::{JsCast, closure::Closure};
use gloo::utils::document;

#[derive(Clone, Properties, PartialEq)]
struct TextBox {
    id: usize,
    title: String,
    content: String,
    image: Option<String>,
}

#[function_component(PaginatedBoxes)]
pub fn paginated_boxes() -> Html {
    const ITEMS_PER_PAGE: usize = 6;
    
    // Generate sample data
    let boxes = use_state(|| {
        (1..=20)
            .map(|i| TextBox {
                id: i,
                title: format!("Article {}", i),
                content: format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. {}.", i),
                image: match i % 3 {
                    0 => Some(format!("img/article{}.jpg", i)),
                    _ => None,
                },
            })
            .collect::<Vec<_>>()
    });
    let current_page = use_state(|| 1);
    let observer = use_state(|| None::<IntersectionObserver>);

    // Calculate pagination
    let total_pages = (*boxes).len() / ITEMS_PER_PAGE + 1;
    let start_index = (*current_page - 1) * ITEMS_PER_PAGE;
    let visible_boxes = &boxes[start_index..usize::min(start_index + ITEMS_PER_PAGE, boxes.len())];
    
    let current_page_clone1 = current_page.clone();
    let current_page_clone2 = current_page.clone();
    let current_page_clone3 = current_page.clone();
    

    let go_to_page = {
        let current_page = current_page.clone();
        move |page: usize| {
            current_page.set(page);
        }
    };

    let go_to_page_clone1 = go_to_page.clone();

    // Intersection observer for lazy loading

    {
        let observer = observer.clone();
        let current_page = current_page.clone();
        
        use_effect_with(*current_page, move |_| {
            let callback = Closure::<dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver)>::wrap(
                Box::new(move |entries, _| {
                    for entry in entries {
                        if entry.is_intersecting() {
                            if let Ok(img) = entry.target().dyn_into::<HtmlImageElement>() {
                                // Load actual image
                                if let Some(src) = img.dataset().get("src") {
                                    //log::info!("Loading image: {}", src); // Debug log
                                    img.set_src(&src);
                                    img.class_list().add_1("loaded").unwrap();
                                }
                            }
                        }
                    }
                })
            );

            let new_observer = IntersectionObserver::new(
                callback.as_ref().unchecked_ref()
            ).unwrap();

            // Cleanup previous observer
            if let Some(old_observer) = &*observer {
                old_observer.disconnect();
            }

            let node_list = document().query_selector_all(".lazy-image").unwrap();
            for index in 0..node_list.length() {
                if let Some(element) = node_list.item(index) {
                    if let Ok(element) = element.dyn_into::<Element>() {
                        new_observer.observe(&element);
                    }
                }
}

            observer.set(Some(new_observer));
            callback.forget();

            || {}
        });
    }


    html! {
        <section class="paginated-section">
        <div class="boxes-grid">
                {visible_boxes.iter().map(|text_box| {
                    html! {
                        <div class="text-box" key={text_box.id}>
                            {text_box.image.as_ref().map(|img_path| {
                                html! {
                                    <div class="box-image-container">
                                        <img
                                            class="lazy-image"
                                            data-src={img_path.clone()}
                                            src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1 1'%3E%3C/svg%3E"
                                            alt={format!("Zdjęcie z {}", text_box.title)}
                                            loading="lazy"
                                            //width="400"
                                            height="300"
                                        />
                                    </div>
                                }
                            }).unwrap_or_default()}
                            
                            <div class="text-content">
                                <h3>{&text_box.title}</h3>
                                <p>{&text_box.content}</p>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            
            <div class="pagination-controls">
                <button 
                    class="pagination-button" 
                    disabled={*current_page == 1}
                    onclick={move |_| go_to_page(*current_page - 1)}
                >
                    {"Poprzednia strona"}
                </button>
                
                <span class="page-indicator">
                    {format!("Strona {:?} z {}", *current_page_clone1, total_pages)}
                </span>
                
                <button 
                    class="pagination-button" 
                    disabled={*current_page_clone2 >= total_pages}
                    onclick={move |_| go_to_page_clone1(*current_page_clone3 + 1)}
                >
                    {"Następna strona"}
                </button>
            </div>

        </section>
    }
}