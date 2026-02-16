use crate::error::{AppError, Result};
use uuid::Uuid;

// ============ Ownership Verification Helpers ============

/// Trait for entities owned by a boss
pub trait OwnedByBoss {
    fn boss_id(&self) -> Uuid;

    /// Verify that the entity is owned by the given boss_id
    fn verify_owner(&self, boss_id: Uuid) -> Result<()> {
        if self.boss_id() != boss_id {
            Err(AppError::Forbidden)
        } else {
            Ok(())
        }
    }
}

/// Trait for entities owned by a user (customer belongs to boss via user_id)
pub trait OwnedByUser {
    fn user_id(&self) -> Uuid;

    fn verify_owner(&self, user_id: Uuid) -> Result<()> {
        if self.user_id() != user_id {
            Err(AppError::Forbidden)
        } else {
            Ok(())
        }
    }
}

impl OwnedByUser for entity::customer::Model {
    fn user_id(&self) -> Uuid {
        self.user_id
    }
}

impl OwnedByBoss for entity::order::Model {
    fn boss_id(&self) -> Uuid {
        self.boss_id
    }
}

impl OwnedByBoss for entity::payroll::Model {
    fn boss_id(&self) -> Uuid {
        self.boss_id
    }
}

impl OwnedByBoss for entity::piece_record::Model {
    fn boss_id(&self) -> Uuid {
        self.boss_id
    }
}
