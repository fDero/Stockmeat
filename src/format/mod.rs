/*---------------------------------------------------------------------+
|  ========================== RUST-MODULE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|       LICENSE: MIT (https://choosealicense.com/licenses/mit/)        |
+---------------------------------------------------------------------*/

pub mod fen_decode;
pub mod fen_encode;
pub mod gui_display;

pub use fen_decode::FenDecode;
pub use fen_encode::FenEncode;
