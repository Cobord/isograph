use std::{collections::BTreeMap, ops::Deref};

use common_lang_types::ServerObjectEntityName;
use pico_macros::legacy_memo;

use crate::{IsographDatabase, NetworkProtocol, RootOperationName};

/// This is a GraphQL-ism and this function should probably not exist.
#[legacy_memo]
pub fn fetchable_types<TNetworkProtocol: NetworkProtocol + 'static>(
    db: &IsographDatabase<TNetworkProtocol>,
) -> Result<
    BTreeMap<ServerObjectEntityName, RootOperationName>,
    TNetworkProtocol::ParseTypeSystemDocumentsError,
> {
    let memo_ref = TNetworkProtocol::parse_type_system_documents(db);
    let (_items, fetchable_types) = memo_ref.deref().as_ref().map_err(|e| e.clone())?;

    Ok(fetchable_types.clone())
}
