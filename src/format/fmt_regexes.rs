/*---------------------------------------------------------------------+
|  ========================== RUST-MODULE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use regex::Regex;
use std::sync::LazyLock;

pub static STARTING_FEN: &str
    = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub static FEN_CASTLING_RGX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"^(?:-|KQ?k?q?|Qk?q?|kq?|q)$"
).unwrap());

pub static UCI_POSITION_RGX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    concat!(
        r"^\s*position\s+",
        r"(?:(?<startpos>startpos)|fen\s+(?<fen>\S+(?:\s+\S+){5}))",
        r"(?:\s+moves(?<moves>(?:\s+[a-h][1-8][a-h][1-8][nbrq]?)*))?",
        r"\s*$",
    )
).unwrap());

pub static UCI_SETTINGS_RGX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    concat!(
        r"^\s*go(?<params>(?:\s+(?:",
        r"(?:wtime|btime|winc|binc|movestogo|depth|nodes|mate|movetime)\s+-?\d+",
        r"|ponder|infinite",
        r"|searchmoves(?:\s+[a-h][1-8][a-h][1-8][nbrq]?)+",
        r"))*)\s*$",
    )
).unwrap());

pub static UCI_SETTINGS_PARAM_RGX: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    concat!(
        r"(?<key>wtime|btime|winc|binc|movestogo|depth|nodes|mate|movetime)\s+(?<value>-?\d+)",
        r"|(?<flag>ponder|infinite)",
        r"|searchmoves(?<moves>(?:\s+[a-h][1-8][a-h][1-8][nbrq]?)+)",
    )
).unwrap());

/*---------------------------------------------------------------------+
|  =========================== UNIT-TESTS ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn castling() {
        for rights in ["-", "K", "q", "Kq", "Qk", "KQk", "KQkq"] {
            assert!(FEN_CASTLING_RGX.is_match(rights), "rejected '{}'", rights);
        }
        for rights in ["", "--", "-K", "KK", "QK", "qk", "x", " KQ"] {
            assert!(!FEN_CASTLING_RGX.is_match(rights), "accepted '{}'", rights);
        }
    }

    #[test]
    fn position_groups() {
        let caps = UCI_POSITION_RGX.captures("  position startpos  ").unwrap();
        assert!(caps.name("startpos").is_some());
        assert!(caps.name("fen").is_none() && caps.name("moves").is_none());

        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        let input = format!("position fen {fen} moves e1g1 a7a8q");
        let caps = UCI_POSITION_RGX.captures(&input).unwrap();
        assert!(caps.name("startpos").is_none());
        assert_eq!(&caps["fen"], fen);
        assert_eq!(&caps["moves"], " e1g1 a7a8q");
    }

    #[test]
    fn position_rejects_invalid() {
        for input in [
            "position", "positionstartpos", "position startpos e2e4",
            "position startpos moves e2e4e7e5", "position startpos moves e9e4",
            "position startpos moves e7e8k", "position fen 8/8/8/8/8/8/8/8 w - - 0",
            "position fen 8/8/8/8/8/8/8/8 w - - 0 1 extra",
        ] {
            assert!(!UCI_POSITION_RGX.is_match(input), "accepted '{}'", input);
        }
    }

    #[test]
    fn settings() {
        for input in [
            "go", "  go \t ", "go ponder wtime 1000 btime -5 movestogo 40",
            "go searchmoves e2e4 d7d8q depth 3 infinite",
        ] {
            assert!(UCI_SETTINGS_RGX.is_match(input), "rejected '{}'", input);
        }
        for input in [
            "gox", "go sideways", "go depth", "go depth 1.5",
            "go depth 5infinite", "go ponderhit", "go searchmoves",
        ] {
            assert!(!UCI_SETTINGS_RGX.is_match(input), "accepted '{}'", input);
        }
    }

    #[test]
    fn settings_params() {
        let caps = UCI_SETTINGS_RGX.captures("go infinite searchmoves e2e4 a7a8n depth 3 wtime -150").unwrap();
        let groups: Vec<_> = UCI_SETTINGS_PARAM_RGX.captures_iter(&caps["params"])
            .map(|param| ["key", "value", "flag", "moves"].map(|name| param.name(name).map(|m| m.as_str())))
            .collect();
        assert_eq!(groups, [
            [None, None, Some("infinite"), None],
            [None, None, None, Some(" e2e4 a7a8n")],
            [Some("depth"), Some("3"), None, None],
            [Some("wtime"), Some("-150"), None, None],
        ]);
    }
}
