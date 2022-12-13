
pub mod home {
    use yew::prelude::*;
    #[function_component(Home)]
    pub fn home() -> Html {
        html! {
            <>
                <div class="bg-video__div">
                    <div class="bg-video__grid">

                        <div class="bgvideo-box__text-box">
                            <h1 class="bgvideo-h1">{"Flat Scooter Riding"}</h1>
                        
                            <p>{"Flat Scooter Riding is in our blood, it's our purpuse,
                                freedom, and inspiration.
                            "}</p>
                            <p>{"Nobody helped us, so we made it ourself.
                            We acted in the darkness for years, and just like crows, we
                            learned to fly."}</p>
                            <p>{"This is also our way to make Flat Scootering something we can work on when we can't ride."}</p>
                        </div>

                        <div class="bgvideo-box__text-box">
                            <h1 class="bgvideo-h1">{"If you want to take the island, burn the boats"}</h1>
                        
                            
                            <p>{"This Brand doesn't want to make Flat riding a trend,
                                this place is meant for us, the flatboys, and everyone in the flat community.
                                No one has cared of us, but we do have each other.
                            "}</p>
                            <p>{"We are going to make it."}</p>
                        
                        </div>

                    </div>
                    
                    <video class="bg-video" width="100%" autoplay=true muted=true loop=true id="myVideo">
                        <source src="img/bgvideo.mp4" type="video/mp4"/>
                    </video>

                </div>
            </>
        }
    }
}