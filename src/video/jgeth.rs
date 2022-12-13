pub mod jgeth {
    #[derive(Clone, PartialEq)]
     pub struct Mafia {
        pub id: i32,
        pub title: String,
        pub rider: String,
        pub url: String
    }

    pub fn getVideos() -> Vec<Mafia> {
      

        vec![
            Mafia {
                id: 1,
                title: "4 Years Of Scootering: Now Dream".to_string(),
                rider: "kyle runo".to_string(),
                url: "https://www.youtube.com/watch?v=C-o18KTpnsY".to_string(),
            },
            Mafia {
                id: 2,
                title: "3 Year Scooter Progression: 2020 Vision (REAL)".to_string(),
                rider: "kyle runo".to_string(),
                url: "https://www.youtube.com/watch?v=1D-0jEiokME".to_string(),
            },
            Mafia {
                id: 3,
                title: "Kalle Korpela Web Edit 2".to_string(),
                rider: "kyle runo".to_string(),
                url: "https://www.youtube.com/watch?v=3MqXuUwP3-Q".to_string(),
            },
            Mafia {
                id: 1,
                title: "The Art of Flat Scooter Riding | OPERATION UNREALISTIC".to_string(),
                rider: "kyle runo".to_string(),
                url: "https://www.youtube.com/watch?v=2W8tpi72Lq0".to_string(),
            },
            Mafia {
                id: 2,
                title: "KALLE KORPELA WEB EDIT 3".to_string(),
                rider: "kyle runo".to_string(),
                url: "https://www.youtube.com/watch?v=Opxyp3RXnXU".to_string(),
            },
            Mafia {
                id: 3,
                title: "Winter 2020 Flat Edit - Gnarliest Flat Scooter Tricks Ever".to_string(),
                rider: "kyle runo".to_string(),
                url: "https://www.youtube.com/watch?v=LNHSukFRda4".to_string(),
            }      

        ]
    }
}