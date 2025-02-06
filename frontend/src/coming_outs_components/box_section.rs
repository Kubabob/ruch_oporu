use yew::prelude::*;

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
    let current_page_clone1 = current_page.clone();
    let current_page_clone2 = current_page.clone();
    let current_page_clone3 = current_page.clone();
    
    // Calculate pagination
    let total_pages = (*boxes).len() / ITEMS_PER_PAGE + 1;
    let start_index = (*current_page - 1) * ITEMS_PER_PAGE;
    let visible_boxes = &boxes[start_index..usize::min(start_index + ITEMS_PER_PAGE, boxes.len())];

    let go_to_page = {
        let current_page = current_page.clone();
        move |page: usize| {
            current_page.set(page);
        }
    };

    let go_to_page_clone1 = go_to_page.clone();

    html! {
        <section class="paginated-section">
            <div class="boxes-grid">
                {visible_boxes.iter().map(|text_box| {
                    html! {
                        <div class="text-box" key={text_box.id}>
                            // Conditional image rendering
                            {if let Some(img_path) = &text_box.image {
                                html! {
                                    <div class="box-image-container">
                                        <img 
                                            src={img_path.clone()} 
                                            alt={format!("Image for {}", text_box.title)} 
                                            class="box-image"
                                        />
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                            
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