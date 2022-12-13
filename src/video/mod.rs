pub mod jgeth;

pub mod video {
    use crate::video::jgeth::jgeth::getVideos;
    use crate::video::jgeth::jgeth::Mafia;
    use yew::prelude::*;
    
    #[function_component(Video)]
    pub fn video() -> Html {

        html!{
            <>  <main>
                    <div class="main-div">
                        <h1>{"Videos"}</h1>
                        <div class="video__grid">
                            {get_videos()}
                        </div>
                    </div>
                </main>
            </>
        }

  
    }

    pub fn get_videos() -> Html {

        
        let videos: Vec<Mafia> = getVideos();

        videos.iter().map(|video|    
            html! {
                <div class="video__div">
                    <img class="video__img" src={get_thumbnail(&video.url)}/>
                    <a class="video__a" href={format!("{}",video.url)}><p class="video__p">{format!("[{}] - {}", video.id, video.title)}</p></a>
                </div>
            }   
        ).collect::<Html>()

    }

    fn get_thumbnail(url: &String) -> String {

        let Some((_, id)) = url.split_once("?v=") else {
             panic!()
        };

        let thumbnail: String = format!("https://img.youtube.com/vi/{}/mqdefault.jpg", id);
        thumbnail
    }

}

