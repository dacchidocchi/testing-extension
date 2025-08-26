use wasi::http::types::{ErrorCode, Fields, Scheme};

use crate::{
    exports::nero::extension::extractor::{
        EpisodesPage, FilterCategory, Guest, SearchFilter, SeriesPage,
    },
    nero::extension::types::{Episode, Series, Video},
    wasi::http::types::OutgoingRequest,
};

wit_bindgen::generate!({
    path: "./wit",
    generate_all,
});

fn calculate_pagination(page: Option<u16>) -> (usize, bool) {
    const ITEMS_PER_PAGE: usize = 12;
    const MAX_PAGES: u16 = 10;

    let current_page = page.unwrap_or(1);
    let offset = (current_page as usize - 1) * ITEMS_PER_PAGE;
    let has_next_page = current_page < MAX_PAGES;

    (offset, has_next_page)
}

struct TestingExtension;

impl TestingExtension {
    fn sample_series() -> Series {
        let outgoing = OutgoingRequest::new(Fields::new());
        outgoing.set_scheme(Some(&Scheme::Https)).unwrap();
        outgoing.set_authority(Some("m.media-amazon.com")).unwrap();
        outgoing.set_path_with_query(Some(
            "/images/M/MV5BMDlmZGJkYTUtNDcwNi00YWMzLTkyNmMtOWQ3MzVhOTU5YWY0XkEyXkFqcGc@._V1_.jpg",
        )).unwrap();

        Series {
            id: "spy-x-family".to_owned(),
            title: "SPY x FAMILY".to_owned(),
            poster_resource: Some(outgoing),
            synopsis: Some(r#"
                World peace is at stake and secret agent Twilight must undergo his most difficult mission 
                yet—pretend to be a family man. Posing as a loving husband and father, he’ll infiltrate an 
                elite school to get close to a high-profile politician. He has the perfect cover, except his 
                wife’s a deadly assassin and neither knows each other’s identity. But someone does, his 
                adopted daughter who’s a telepath!
            "#.to_owned()),
            type_: Some("Series".to_owned()),
        }
    }

    fn sample_episode() -> Episode {
        let outgoing = OutgoingRequest::new(Fields::new());
        outgoing.set_scheme(Some(&Scheme::Https)).unwrap();
        outgoing.set_authority(Some("m.media-amazon.com")).unwrap();
        outgoing.set_path_with_query(Some(
            "/images/M/MV5BOTAxODZiYzAtOGRhOC00ZjlmLTkxYzEtMGZjNDE0MjcwZTc1XkEyXkFqcGc@._V1_.jpg",
        )).unwrap();

        Episode {
            id: "1".to_owned(),
            number: 1,
            title: Some("OPERATION STRIX".to_owned()),
            thumbnail_resource: Some(outgoing),
            description: Some(r#"
                Twilight is an agent that works for WISE, Westalis's intelligence agency, and he is tasked with 
                investigating Desmond, who is in Ostania and planning to start a war. Twilight disguises himself 
                as the psychiatrist Loid Forger and adopts a girl named Anya so that he can enroll her into the 
                prestigious Eden College to get closer to his target. Unbeknownst to him, Anya is actually a 
                telepath who can read people's minds. One day, members of a mafia group that is after Twilight 
                kidnaps Anya. Loid realizes that he needs to reconsider his priorities and...
            "#.to_owned()),
        }
    }

    fn sample_video() -> Video {
        let outgoing = OutgoingRequest::new(Fields::new());
        outgoing.set_scheme(Some(&Scheme::Https)).unwrap();
        outgoing
            .set_authority(Some("commondatastorage.googleapis.com"))
            .unwrap();
        outgoing
            .set_path_with_query(Some("/gtv-videos-bucket/sample/BigBuckBunny.mp4"))
            .unwrap();

        Video {
            http_resource: outgoing,
            server: "Google".to_owned(),
            resolution: (0, 0),
        }
    }
}

#[allow(unused_variables)]
impl Guest for TestingExtension {
    fn filters() -> Result<Vec<FilterCategory>, ErrorCode> {
        Err(ErrorCode::InternalError(Some("Not implemented".to_owned())))
    }

    fn search(
        query: String,
        page: Option<u16>,
        filters: Vec<SearchFilter>,
    ) -> Result<SeriesPage, ErrorCode> {
        let (offset, has_next_page) = calculate_pagination(page);
        let page = SeriesPage {
            series: (1..=12)
                .map(|i| Series {
                    id: (offset + i).to_string(),
                    ..Self::sample_series()
                })
                .collect::<Vec<_>>(),
            has_next_page,
        };
        Ok(page)
    }

    fn get_series_info(series_id: String) -> Result<Series, ErrorCode> {
        Ok(Self::sample_series())
    }

    fn get_series_episodes(
        series_id: String,
        page: Option<u16>,
    ) -> Result<EpisodesPage, ErrorCode> {
        let (offset, has_next_page) = calculate_pagination(page);
        let page = EpisodesPage {
            episodes: (1..=12)
                .map(|i| Episode {
                    id: (offset + i).to_string(),
                    number: i as u16,
                    ..Self::sample_episode()
                })
                .collect(),
            has_next_page,
        };
        Ok(page)
    }

    fn get_series_videos(series_id: String, episode_id: String) -> Result<Vec<Video>, ErrorCode> {
        Ok(vec![Self::sample_video()])
    }
}

export!(TestingExtension);
