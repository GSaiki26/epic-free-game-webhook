// Libs
use chrono::{DateTime, FixedOffset};

use super::epic_catalog::StoreGame;

impl StoreGame {
    /**
     * A method to get the dates from the offer.
     */
    pub fn get_offer_datetimes(&self) -> Option<(DateTime<FixedOffset>, DateTime<FixedOffset>)> {
        // Get the game promotion time.
        let offer = self
            .promotions
            .promotional_offers
            .first()?
            .promotional_offers
            .first()
            .unwrap();
        Some((
            DateTime::parse_from_rfc3339(&offer.start_date).unwrap(),
            DateTime::parse_from_rfc3339(&offer.end_date).unwrap(),
        ))
    }
}
