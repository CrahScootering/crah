pub mod template {

    use yew::prelude::*;
    use crate::router::Route;
    use yew_router::prelude::Link;

    fn print_element() -> Html {

        #[derive(Clone, Properties, PartialEq)]
        struct Element {
            name: String,
            route: Route
        }

        let elements: Vec<Element> = vec![
            Element {
                name: "Home".to_string(),
                route: Route::Home,
            },
            Element {
                name: "Videos".to_string(),
                route: Route::Videos,
            },
            Element {
                name: "Info".to_string(),
                route: Route::Flatboys,
            },
        ];

        elements.iter().map(|element|
            html! {
                <li class="header__li">
                    <Link<Route> to={{element.route}}>{format!("{}", element.name)}</Link<Route>>
                    //<a href={format!("{}", element.name.to_lowercase())}>{format!("{}", element.name)}</a>
                </li>
            }
        ).collect::<Html>()

    }

    #[function_component(Header)]
    pub fn header() -> Html {

        html! {
            <header class="header">
                <nav class="header__nav">
                    <ul class="header__ul">
                        {print_element()}
                    </ul>
                </nav>
            </header>
        }

    }

    fn create_footer() -> Html {
        let sezioni: Vec<(&str, Vec<&str>)> = vec![
            ( "Resources",
            vec![
                "Extra Content",
                "Branding on Behance",
                "BrandBook",
            ]),
            ( "Work",
            vec![
                "Work for us",
                "App",
                "Sponsor",
            ]),
        ];

        sezioni.iter().map(|sezione| {

            let (titolo, lista) = sezione;
            
            fn stampalista(lista: &&Vec<&str>) -> Html {

                let mut x: i8 = 0;

                let mut z: Html = html!{

                };

                for l in lista.iter() {

                    x = x + 1;

                    z = html!{
                        <>
                            {html_nested! {z}}
                            <a class="footer__a">
                                <li class="footer__li">{l}</li>
                            </a>
                        </>
                    };
                    

                }

                if x == 0 {
                    return html!{};
                } else {
                    return z;
                }
                
            }
        

            html! {
                <>
                    <div class="footer__div">
                        <h2 class="footer__h2">{titolo}</h2>
                        <ul>
                            {stampalista(&lista)}
                        </ul>
                    </div>
                </>
            }

          }
        ).collect::<Html>()

    }
    
    #[function_component(Footer)]
    pub fn footer() -> Html {

        html! {
            <>
                <footer class="footer">
                    <nav class="footer__nav">
                        {create_footer()}
                    </nav>
                </footer>
            </>
        }

    }

}