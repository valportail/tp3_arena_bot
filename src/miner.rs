// ─── Partie 2 : Pool de mineurs ──────────────────────────────────────────────
//
// Objectif : créer un pool de N threads qui cherchent des nonces en parallèle.
//
// Concepts exercés : thread::spawn, mpsc::channel, Arc, move closures.
//
// Architecture :
//
//   Thread principal                        Threads mineurs (x N)
//        |                                        |
//        |── mpsc::Sender<MineRequest> ──────────>|  (challenges à résoudre)
//        |                                        |
//        |<── mpsc::Sender<MineResult> ──────────>|  (solutions trouvées)
//        |                                        |
//
// Chaque thread mineur :
//   1. Attend un MineRequest sur son channel
//   2. Appelle pow::pow_search() avec un start_nonce différent
//   3. Si un nonce est trouvé, envoie un MineResult
//
// ─────────────────────────────────────────────────────────────────────────────

use crate::pow;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use uuid::Uuid;

/// Requête de minage envoyée aux threads mineurs.
#[derive(Debug, Clone)]
pub struct MineRequest {
    pub seed: String,
    pub tick: u64,
    pub resource_id: Uuid,
    pub agent_id: Uuid,
    pub target_bits: u8,
}

/// Résultat renvoyé par un mineur quand il trouve un nonce valide.
#[derive(Debug, Clone)]
pub struct MineResult {
    pub tick: u64,
    pub resource_id: Uuid,
    pub nonce: u64,
}

// TODO: Définir la structure MinerPool.
//
// Elle doit contenir :
//   - Le Sender pour envoyer des MineRequest aux threads
//   - Le Receiver pour récupérer les MineResult
//
// Indice : les types sont :
//   std::sync::mpsc::Sender<MineRequest>
//   std::sync::mpsc::Receiver<MineResult>
//
// pub struct MinerPool {
//     ...
// }

pub struct MinerPool {
    sender: std::sync::mpsc::Sender<MineRequest>,
    receiver: std::sync::mpsc::Receiver<MineResult>,
}

// TODO: Implémenter MinerPool.
//
// impl MinerPool {
//     /// Crée un pool de `n` threads mineurs.
//     ///
//     /// Chaque thread :
//     ///   1. Possède un Receiver<MineRequest> (partagé via Arc<Mutex<>>)
//     ///   2. Possède un Sender<MineResult> (cloné pour chaque thread)
//     ///   3. Boucle : recv() → pow_search() → send() si trouvé
//     ///
//     /// Indices :
//     ///   - Un seul Receiver existe par channel. Pour le partager entre N threads,
//     ///     il faut le wrapper dans Arc<Mutex<Receiver<MineRequest>>>.
//     ///   - Chaque thread clone le Arc pour accéder au Receiver.
//     ///   - pow::pow_search() prend un start_nonce et un batch_size.
//     ///     Utilisez rand::random::<u64>() comme start_nonce pour que chaque
//     ///     appel explore une zone différente.
//     ///   - Batch size recommandé : 100_000
//     ///
//     pub fn new(n: usize) -> Self {
//         // Créer les 2 channels :
//         //   - (request_tx, request_rx) pour envoyer les challenges
//         //   - (result_tx, result_rx) pour recevoir les solutions
//         //
//         // Wrapper request_rx dans Arc<Mutex<>>
//         //
//         // Pour chaque thread (0..n) :
//         //   - Cloner le Arc<Mutex<Receiver<MineRequest>>>
//         //   - Cloner le result_tx
//         //   - thread::spawn(move || { ... boucle de minage ... })
//         //
//         // Retourner MinerPool { request_tx, result_rx }
//         todo!()
//     }
//
//     /// Envoie un challenge de minage au pool.
//     pub fn submit(&self, request: MineRequest) {
//         todo!()
//     }
//
//     /// Tente de récupérer un résultat sans bloquer.
//     pub fn try_recv(&self) -> Option<MineResult> {
//         todo!()
//     }
// }

impl MinerPool {
    pub fn new(n: usize) -> Self {
        let (request_tx, request_rx) = mpsc::channel::<MineRequest>();
        let (result_tx, result_rx) = mpsc::channel::<MineResult>();

        let request_rx = Arc::new(Mutex::new(request_rx));

        for _ in 0..n {
            let request_rx = Arc::clone(&request_rx);
            let result_tx = result_tx.clone();

            thread::spawn(move || loop {
                let reception = {
                    let lock = request_rx.lock().unwrap();
                    lock.recv()
                };

                match reception {
                    Ok(request) => {
                        if let Some(p_nonce) = pow::pow_search(
                            &request.seed,
                            request.tick,
                            request.resource_id,
                            request.agent_id,
                            request.target_bits,
                            rand::random::<u64>(),
                            100000,
                        ) {
                            let result = MineResult {
                                tick: request.tick,
                                resource_id: request.resource_id,
                                nonce: p_nonce,
                            };
                            result_tx.send(result).unwrap();
                        }
                    }
                    Err(_) => break,
                }
            });
        }

        MinerPool {
            sender: request_tx,
            receiver: result_rx,
        }
    }

    pub fn submit(&self, request: MineRequest) {
        self.sender.send(request).unwrap();
    }

    pub fn try_rcv(&self) -> Option<MineResult> {
        Some(self.receiver.recv().unwrap())
    }
}
