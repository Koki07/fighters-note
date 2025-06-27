#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250623_121248_posts;
mod m20250623_134244_character_notes;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250623_121248_posts::Migration),
            Box::new(m20250623_134244_character_notes::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}