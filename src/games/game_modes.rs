use std::fmt;

#[derive(Debug)]
pub enum GameMode {
    Solo,
    VragenEnMeegaan,
    Troel,
    Piccolo,
    KleineMiserie,
    GroteMiserie,
    MiserieOpTafel,
    Abondance,
    SoloSlim,
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameMode::Solo => write!(f, "Solo"),
            GameMode::VragenEnMeegaan => write!(f, "Vragen en meegaan"),
            GameMode::Troel => write!(f, "Troel"),
            GameMode::Piccolo => write!(f, "Piccolo"),
            GameMode::KleineMiserie => write!(f, "Kleine Miserie"),
            GameMode::GroteMiserie => write!(f, "Grote Miserie"),
            GameMode::MiserieOpTafel => write!(f, "Miserie op tafel"),
            GameMode::Abondance => write!(f, "Abondance"),
            GameMode::SoloSlim => write!(f, "Solo slim"),
        }
    }
}
