//! Generic db abstration.

pub mod error;
#[macro_use]
pub mod mock;
pub mod mysql;
pub mod params;
pub mod results;
pub mod util;

use futures::future::Future;

pub use self::error::DbError;

lazy_static! {
    /// For efficiency, it's possible to use fixed pre-determined IDs for
    /// common collection names.  This is the canonical list of such
    /// names.  Non-standard collections will be allocated IDs starting
    /// from the highest ID in this collection.
    static ref STD_COLLS: Vec<(i32, &'static str)> = {
        vec![
        (1, "clients"),
        (2, "crypto"),
        (3, "forms"),
        (4, "history"),
        (5, "keys"),
        (6, "meta"),
        (7, "bookmarks"),
        (8, "prefs"),
        (9, "tabs"),
        (10, "passwords"),
        (11, "addons"),
        (12, "addresses"),
        (13, "creditcards"),
        ]
    };
}

type DbFuture<T> = Box<Future<Item = T, Error = DbError>>;

pub trait Db: Send {
    // XXX: add a generic fn transaction(&self, f)

    fn get_collection_id(
        &self,
        params: &params::GetCollectionId,
    ) -> DbFuture<results::GetCollectionId>;

    fn get_collections(&self, params: &params::GetCollections)
        -> DbFuture<results::GetCollections>;

    fn get_collection_counts(
        &self,
        params: &params::GetCollectionCounts,
    ) -> DbFuture<results::GetCollectionCounts>;

    fn get_collection_usage(
        &self,
        params: &params::GetCollectionUsage,
    ) -> DbFuture<results::GetCollectionUsage>;

    fn get_storage_usage(
        &self,
        params: &params::GetStorageUsage,
    ) -> DbFuture<results::GetStorageUsage>;

    fn delete_all(&self, params: &params::DeleteAll) -> DbFuture<results::DeleteAll>;

    fn delete_collection(
        &self,
        params: &params::DeleteCollection,
    ) -> DbFuture<results::DeleteCollection>;

    fn get_collection(&self, params: &params::GetCollection) -> DbFuture<results::GetCollection>;

    fn post_collection(&self, params: &params::PostCollection)
        -> DbFuture<results::PostCollection>;

    fn delete_bso(&self, params: &params::DeleteBso) -> DbFuture<results::DeleteBso>;

    fn get_bso(&self, params: &params::GetBso) -> DbFuture<results::GetBso>;

    fn put_bso(&self, params: &params::PutBso) -> DbFuture<results::PutBso>;
}

#[derive(Debug)]
pub enum Sorting {
    None,
    Newest,
    Oldest,
    Index,
}