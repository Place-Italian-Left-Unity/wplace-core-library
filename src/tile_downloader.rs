use image::DynamicImage;

use crate::GenericBytes;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Tile {
    x: u16,
    y: u16,
}

impl Tile {
    fn url(self) -> String {
        format!(
            "https://backend.wplace.live/files/s0/tiles/{}/{}.png",
            self.x, self.y
        )
    }
    const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

struct DownloadData {
    image: DynamicImage,
    last_download: std::time::SystemTime,
}

type TileDownloadCache = std::collections::HashMap<Tile, DownloadData>;
static mut TILE_DOWNLOAD_CACHE: std::sync::LazyLock<std::sync::RwLock<TileDownloadCache>> =
    std::sync::LazyLock::new(|| std::sync::RwLock::new(std::collections::HashMap::new()));

pub enum TileDownloadError {}

/// This whole struct was made to handle a the TILE_DOWNLOAD_CACHE side effects
pub struct TileDownloader;
impl TileDownloader {
    fn clean_cache() {
        unsafe {
            #[allow(static_mut_refs)]
            TILE_DOWNLOAD_CACHE
                .get_mut()
                .unwrap()
                .retain(|_, v| v.last_download.elapsed().unwrap().as_secs() < 360);
        }
    }

    fn get_from_cache(v: Tile) -> Option<DynamicImage> {
        unsafe {
            #[allow(static_mut_refs)]
            TILE_DOWNLOAD_CACHE
                .read()
                .unwrap()
                .get(&v)
                .map(|v| v.image.clone())
        }
    }

    pub(crate) fn download(tile_x: u16, tile_y: u16) -> Result<DynamicImage, image::ImageError> {
        Self::clean_cache();
        let tile = Tile::new(tile_x, tile_y);

        if let Some(image) = Self::get_from_cache(tile) {
            return Ok(image);
        }

        let mut curl_client = curl::easy::Easy2::new(GenericBytes(Vec::with_capacity(2_000_000)));
        curl_client.url(&tile.url()).expect("Couldn't select url");
        curl_client.perform().expect("Couldn't perform");

        image::load_from_memory_with_format(&curl_client.get_ref().0, image::ImageFormat::Png)
    }
}
