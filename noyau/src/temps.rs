//! Constantes de temps et comparaison des heures de reset.

pub const JOUR_S: i64 = 86_400;
pub const SEMAINE_S: i64 = 7 * JOUR_S;

/// Écart toléré entre deux lectures du même reset : le même reset hebdomadaire a été lu
/// `…:00:00.061Z` puis `…:59:59.531Z` (14/10/2026), soit une gigue d'environ une demi-seconde.
const TOLERANCE_RESET_S: i64 = 120;

/// Heure arrondie à la minute la plus proche : Claude Code donne le reset à `10:59:59`, c'est celui
/// de 11:00 (sinon l'appli afficherait « 12:59 » au lieu de « 13:00 »).
pub fn arrondi_minute(t: i64) -> i64 {
    (t + 30).div_euclid(60) * 60
}

/// Vrai si les deux heures de reset (secondes Unix) désignent le même reset.
pub fn meme_reset(a: i64, b: i64) -> bool {
    (a - b).abs() < TOLERANCE_RESET_S
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gigue_d_une_seconde_est_le_meme_reset() {
        // 2026-10-15T11:00:00Z, lu avec 1 s d'écart dans les deux sens
        let r = 1_792_062_000;
        assert!(meme_reset(r, r));
        assert!(meme_reset(r, r - 1));
        assert!(meme_reset(r + 1, r));
    }

    #[test]
    fn le_reset_est_arrondi_a_la_minute() {
        let r = 1_792_062_000;
        assert_eq!(arrondi_minute(r - 1), r); // 10:59:59 → 11:00
        assert_eq!(arrondi_minute(r + 1), r);
        assert_eq!(arrondi_minute(r), r);
        assert_eq!(arrondi_minute(r + 29), r);
        assert_eq!(arrondi_minute(r + 31), r + 60);
    }

    #[test]
    fn une_semaine_d_ecart_n_est_pas_le_meme_reset() {
        let r = 1_792_062_000;
        assert!(!meme_reset(r, r + SEMAINE_S));
        assert!(!meme_reset(r, r + 3_600));
    }
}
