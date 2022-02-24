use std::option::Option;
use anyhow::{Error};
use rand::Rng;
use sha2::{Sha256, Digest};
use crate::{ChallengeRequestMessage, ChallengeSequence, ChallengeSolution};

#[derive(Clone, Debug)]
pub struct Challenge {
    pub difficulty: u8,
    pub hash_seq: ChallengeSequence,
    pub hash: Sha256,
    pub solution: Option<ChallengeSolution>,
}

impl Challenge {
    pub fn new(difficulty: u8) -> Self {
        let rand_seq = rand::thread_rng().gen::<ChallengeSequence>();
        let mut hash = Sha256::new();
        hash.update(rand_seq);
        Self {
            difficulty,
            hash_seq: rand_seq,
            hash,
            solution: None
        }
    }

    pub fn of(crm: ChallengeRequestMessage) -> Self {
        let mut hash = Sha256::new();
        hash.update(crm.hash_seq);
        Self {
            difficulty: crm.difficulty,
            hash_seq: crm.hash_seq,
            hash,
            solution: None,
        }
    }

    pub fn solve(&mut self) -> Result<(), Error> {
        log::info!("Solving challenge");
        let mut rng = rand::thread_rng();
        let mut tries: u128 = 0;
        loop {
            let s = rng.gen::<ChallengeSolution>();
            tries += 1;
            if let Ok(()) = self.verify(&s) {
                log::info!("Solving took {:?} tries", tries);
                self.solution = Some(s);
                return Ok(())
            }
        }
    }

    pub fn verify(&self, solution: &ChallengeSolution) -> Result<(), Error> {
        let mut hash = self.hash.clone();
        hash.update(solution);
        let res = hash.finalize();
        let mut leading_zeros = 0;

        for c in res[..].iter() {
            match *c {
                0 => leading_zeros += 1,
                _ => break
            };
        }

        if leading_zeros >= self.difficulty {
            return Ok(())
        }
        Err(Error::msg("Challenge is not passed"))
    }
}
