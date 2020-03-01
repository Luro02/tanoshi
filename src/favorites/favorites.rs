use crate::favorites::{AddFavoritesResponse, FavoriteManga, GetFavoritesResponse};
use sled::Db;

#[derive(Clone)]
pub struct Favorites {
    db: Db,
}

impl Favorites {
    pub fn new() -> Self {
        let db = sled::open("./db/favorites").unwrap();
        Favorites { db }
    }

    pub fn get_favorites(&self, username: String) -> GetFavoritesResponse {
        let mangas: Vec<FavoriteManga> = match self
            .db
            .get(format!("favorites:{}:mangas", username))
            .unwrap()
        {
            Some(bytes) => serde_json::from_slice(&bytes).unwrap(),
            None => vec![],
        };
        GetFavoritesResponse {
            favorites: Some(mangas),
            status: "success".to_string(),
        }
    }

    pub fn add_favorite(&self, username: String, manga: FavoriteManga) -> AddFavoritesResponse {
        let status = match self.db.fetch_and_update(
            format!("favorites:{}:mangas", username),
            |fav: Option<&[u8]>| {
                let mut mangas: Vec<FavoriteManga> = match fav {
                    Some(bytes) => {
                        let manga_fav: Vec<FavoriteManga> = serde_json::from_slice(bytes).unwrap();
                        manga_fav
                    }
                    None => vec![],
                };
                if !mangas.contains(&manga) {
                    mangas.push(manga.clone());
                }
                serde_json::to_vec(&mangas).ok()
            },
        ) {
            Ok(_) => "success".to_string(),
            Err(_) => "failed".to_string(),
        };
        AddFavoritesResponse { status }
    }

    pub fn remove_favorites(&self, username: String, manga: FavoriteManga) -> AddFavoritesResponse {
        let status = match self.db.fetch_and_update(
            format!("favorites:{}:mangas", username),
            |fav: Option<&[u8]>| {
                let mut mangas: Vec<FavoriteManga> = match fav {
                    Some(bytes) => {
                        let manga_fav: Vec<FavoriteManga> = serde_json::from_slice(bytes).unwrap();
                        manga_fav
                    }
                    None => vec![],
                };

                match mangas.iter().position(|m| m.clone() == manga) {
                    Some(index) => Some(mangas.remove(index)),
                    None => None,
                };
                serde_json::to_vec(&mangas).ok()
            },
        ) {
            Ok(_) => "success".to_string(),
            Err(_) => "failed".to_string(),
        };
        AddFavoritesResponse { status }
    }
}
