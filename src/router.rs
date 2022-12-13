use yew_router::prelude::*;
use yew::prelude::*;

use crate::template::template::Header;
use crate::template::template::Footer;

use crate::home::home::Home;
use crate::video::video::Video;


#[derive(Debug, Clone, PartialEq, Routable, Copy)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/videos")]
    Videos,
    #[at("/flatboys")]
    Flatboys,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html!{
            <> 
                
                <Header/>   
                <Home/>
                <Footer/>
                
            </>
        },
        Route::Videos => html!{
            <>
                <Header/>
                <Video/>
                <Footer/>
            </>
        },
        Route::Flatboys => html!{
            <>
                <Header/>
                <Footer/>
            </>
        }
    }
}