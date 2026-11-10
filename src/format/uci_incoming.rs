/*---------------------------------------------------------------------+
|  ========================== SOURCE-CODE ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

use anyhow::{Context, Result};
use anyhow::{bail, ensure};
use std::time::Duration;

use crate::format::fmt_regexes::*;
use crate::format::*;
use crate::model::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UciIncoming {
    Uci, IsReady, UciNewGame, PonderHit, Stop,
    Position(Game), Go(SearchSettings), Quit,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchSettings {
    pub searchmoves: Vec<Move>,
    pub ponder: bool,
    pub wtime: Option<Duration>,
    pub btime: Option<Duration>,
    pub winc: Option<Duration>,
    pub binc: Option<Duration>,
    pub movestogo: Option<u64>,
    pub depth: Option<u64>,
    pub nodes: Option<u64>,
    pub movetime: Option<Duration>,
    pub infinite: bool,
}

pub trait UciDecode: Sized {
    fn uci_decode(line: &str) -> Result<UciIncoming>;
}

impl UciDecode for UciIncoming {
    fn uci_decode(input: &str) -> Result<UciIncoming> {
        let error_msg = "Unrecognized/Unsupported UCI command";
        match input.split_whitespace().next() {
            Some("uci")        => Ok(UciIncoming::Uci),
            Some("isready")    => Ok(UciIncoming::IsReady),
            Some("ucinewgame") => Ok(UciIncoming::UciNewGame),
            Some("position")   => Game::uci_decode(input),
            Some("go")         => SearchSettings::uci_decode(input),
            Some("ponderhit")  => Ok(UciIncoming::PonderHit),
            Some("stop")       => Ok(UciIncoming::Stop),
            Some("quit")       => Ok(UciIncoming::Quit),
            _                  => bail!(error_msg),
        }
    }
}

impl UciDecode for Game {
    fn uci_decode(input: &str) -> Result<UciIncoming> {
        let error_msg = "Illformed UCI 'position' command";
        let caps = UCI_POSITION_RGX.captures(input).context(error_msg)?;
        let mut game = match caps.name("fen") {
            Some(fen) => Game::fen_decode(fen.as_str())?,
            None => Game::default(),
        };
        let moves = caps.name("moves").map_or("", |m| m.as_str());
        for text in moves.split_whitespace() {
            let mut legal_moves = game.get_legal_moves().into_iter();
            let target = legal_moves.find(|mov| mov.to_string() == text);
            ensure!(target.is_some(), error_msg);
            game = game.apply_unchecked(target.unwrap());
        }
        Ok(UciIncoming::Position(game))
    }
}

impl UciDecode for SearchSettings {
    fn uci_decode(input: &str) -> Result<UciIncoming> {
        let error_msg = "Illformed UCI 'go' command";
        let caps = UCI_SETTINGS_RGX.captures(input).context(error_msg)?;
        let mut settings = SearchSettings::default();
        for param in UCI_SETTINGS_PARAM_RGX.captures_iter(&caps["params"]) {
            let key_or_flag = param.name("key").or(param.name("flag"));
            let Some(struct_field_name) = key_or_flag else {
                continue
            };
            let value_or_zero = param.name("value").map_or("0", |v| v.as_str());
            let number: i64 = value_or_zero.parse().context(error_msg)?;
            let millis = Duration::from_millis(number.max(0) as u64);
            let count = || u64::try_from(number).context(error_msg);
            match struct_field_name.as_str() {
                "ponder"    => settings.ponder    = true,
                "infinite"  => settings.infinite  = true,
                "wtime"     => settings.wtime     = Some(millis),
                "btime"     => settings.btime     = Some(millis),
                "winc"      => settings.winc      = Some(millis),
                "binc"      => settings.binc      = Some(millis),
                "movetime"  => settings.movetime  = Some(millis),
                "movestogo" => settings.movestogo = Some(count()?),
                "depth"     => settings.depth     = Some(count()?),
                "nodes"     => settings.nodes     = Some(count()?),
                _           => {},
            }
        }
        Ok(UciIncoming::Go(settings))
    }
}

/*---------------------------------------------------------------------+
|  =========================== UNIT-TESTS ============================ |
|     Author: fDero (francescodero@outlook.it - github.com/fDero)      |
|               LICENSE: MIT (https://mit-license.org)                 |
+---------------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;

    fn go(input: &str) -> SearchSettings {
        match UciIncoming::uci_decode(input).unwrap() {
            UciIncoming::Go(settings) => settings,
            other => panic!("expected Go, got {:?}", other),
        }
    }

    #[test]
    fn go_without_params() {
        assert_eq!(go("go"), SearchSettings::default());
        assert_eq!(go("  go  \t "), SearchSettings::default());
    }

    #[test]
    fn go_clock() {
        let settings = go("go wtime 60000 btime 59000 winc 1000 binc 500 movestogo 40");
        assert_eq!(settings.wtime, Some(Duration::from_millis(60000)));
        assert_eq!(settings.btime, Some(Duration::from_millis(59000)));
        assert_eq!(settings.winc, Some(Duration::from_millis(1000)));
        assert_eq!(settings.binc, Some(Duration::from_millis(500)));
        assert_eq!(settings.movestogo, Some(40));
        assert!(!settings.infinite && !settings.ponder);
    }

    #[test]
    fn go_limits_and_flags() {
        let settings = go("go   depth 7 nodes 100000 \t mate 3 movetime 250 ponder infinite");
        assert_eq!(settings.depth, Some(7));
        assert_eq!(settings.nodes, Some(100000));
        assert_eq!(settings.movetime, Some(Duration::from_millis(250)));
        assert!(settings.ponder && settings.infinite);
    }

    #[test]
    fn go_mate_is_accepted_but_ignored() {
        let settings = go("go mate 3 depth 5");
        assert_eq!(settings, SearchSettings { depth: Some(5), ..Default::default() });
    }

    #[test]
    fn go_negative_time_is_clamped() {
        assert_eq!(go("go wtime -150 btime 1000").wtime, Some(Duration::ZERO));
    }

    #[test]
    fn go_searchmoves_is_skipped() {
        let settings = go("go searchmoves e2e4 d2d4 e7e8q depth 3");
        assert!(settings.searchmoves.is_empty());
        assert_eq!(settings.depth, Some(3));
    }

    #[test]
    fn position_rejects_illegal_fen() {
        assert!(UciIncoming::uci_decode("position fen 8/8/8/8/8/8/8/8 w - - 0 1").is_err());
        assert!(UciIncoming::uci_decode("position fen 4k3/8/8/8/8/8/8/8 w - - 0 1 moves e8e7").is_err());
    }

    #[test]
    fn go_rejects_malformed() {
        assert!(UciIncoming::uci_decode("go depth").is_err());
        assert!(UciIncoming::uci_decode("go depth x").is_err());
        assert!(UciIncoming::uci_decode("go depth -1").is_err());
        assert!(UciIncoming::uci_decode("go depth 5infinite").is_err());
        assert!(UciIncoming::uci_decode("go sideways").is_err());
    }
}
