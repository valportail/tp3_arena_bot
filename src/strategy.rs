// ─── Partie 3 : Stratégie de déplacement ─────────────────────────────────────
//
// Objectif : définir un trait Strategy et l'utiliser via Box<dyn Strategy>
// pour choisir le prochain mouvement du bot à chaque tick.
//
// Concepts exercés : dyn Trait, Box<dyn Strategy>, Send, dispatch dynamique.
//
// ─────────────────────────────────────────────────────────────────────────────

// Importer les types nécessaires de state.rs
use crate::state::GameState;

// Définir le trait Strategy.
//
// Le trait doit :
//   - Être object-safe (pas de generics dans les méthodes)
//   - Être Send (pour pouvoir être utilisé dans un contexte multi-thread)
//   - Avoir une méthode next_move qui retourne un déplacement optionnel

pub trait Strategy: Send {
    /// Décide du prochain mouvement en fonction de l'état du jeu.
    ///
    /// Retourne Some((dx, dy)) avec dx, dy ∈ {-1, 0, 1}, ou None pour rester sur place.
    fn next_move(&self, state: &GameState) -> Option<(i8, i8)>;
}

// Implémenter NearestResourceStrategy.
//
// Cette stratégie se dirige vers la ressource la plus proche (distance de Manhattan).

pub struct NearestResourceStrategy;

impl Strategy for NearestResourceStrategy {
    fn next_move(&self, state: &GameState) -> Option<(i8, i8)> {
        // 1. Trouver la ressource la plus proche en distance de Manhattan :
        //    distance = |resource.x - position.x| + |resource.y - position.y|
        //
        //    Indice : utilisez .iter().min_by_key(|r| ...)

        let Some(nearest_resource) = state
            .resources
            .iter()
            .min_by_key(|r| r.x.abs_diff(state.position.0) + r.y.abs_diff(state.position.1))
        else {
            return None;
        };

        // 2. Calculer la direction (dx, dy) vers cette ressource :
        //    - Si resource.x > position.x → dx = 1
        //    - Si resource.x < position.x → dx = -1
        //    - Sinon dx = 0
        //    - Idem pour dy
        //
        //    Indice : utilisez i16 pour les calculs puis .signum() puis cast en i8

        let dx = if nearest_resource.x > state.position.0 {
            1
        } else if nearest_resource.x < state.position.0 {
            -1
        } else {
            0
        };
        let dy = if nearest_resource.y > state.position.1 {
            1
        } else if nearest_resource.y < state.position.1 {
            -1
        } else {
            0
        };

        // 3. Retourner Some((dx, dy)), ou None si aucune ressource
        Some((dx, dy))
    }
}

// ─── BONUS : Implémenter d'autres stratégies ────────────────────────────────
//
// Exemples :
//   - RandomStrategy : mouvement aléatoire
//   - FleeStrategy : s'éloigne des autres agents
//   - HybridStrategy : combine plusieurs stratégies
//
// Utilisation dans main.rs :
//   let strategy: Box<dyn Strategy> = Box::new(NearestResourceStrategy);
//
// On peut changer de stratégie sans modifier le reste du code grâce au
// dispatch dynamique (Box<dyn Strategy>).
