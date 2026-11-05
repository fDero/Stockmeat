/*---------------------------------------------------------------------+
|  ========================== RUST-MODULE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

pub mod fen_decode;
pub mod fen_encode;
pub mod fmt_regexes;
pub mod ascii_display;
pub mod uci_incoming;
pub mod uci_outgoing;

pub use fen_decode::FenDecode;
pub use fen_encode::FenEncode;
pub use fmt_regexes::STARTING_FEN;
