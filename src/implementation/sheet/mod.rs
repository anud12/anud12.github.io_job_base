mod get;
mod parse_response;
mod save;
mod sheet;

use crate::GoogleSession;

pub struct Sheet {
    session: GoogleSession,
    spreadsheet_id: String,
}
