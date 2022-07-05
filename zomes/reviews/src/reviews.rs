use std::collections::BTreeMap;
use devhub_types::{
    AppResult, UpdateEntityInput, GetEntityInput,
    dnarepo_entry_types::{
	ReviewEntry,
    },
    fmt_path,
};
use hc_crud::{
    now, create_entity, get_entity, update_entity,
    Entity, // UtilsError,
};
use hdk::prelude::*;

use crate::constants::{
    LT_NONE,
    TAG_REVIEW,
    ANCHOR_REVIEWS,
};




#[derive(Debug, Deserialize)]
pub struct ReviewInput {
    pub subject_ids: Vec<EntryHash>,
    pub accuracy_rating: u8,
    pub efficiency_rating: u8,
    pub message: String,

    // optional
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}

pub fn create_review(input: ReviewInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Creating Review for: subject IDs {:?} ", input.subject_ids );
    let pubkey = agent_info()?.agent_initial_pubkey;
    let default_now = now()?;

    let review = ReviewEntry {
	subject_ids: input.subject_ids.to_owned(),
	author: pubkey.clone(),
	accuracy_rating: input.accuracy_rating,
	efficiency_rating: input.efficiency_rating,
	message: input.message,
	published_at: input.published_at
	    .unwrap_or( default_now ),
	last_updated: input.last_updated
	    .unwrap_or( default_now ),
	metadata: input.metadata
	    .unwrap_or( BTreeMap::new() ),
	deleted: false,
    };

    let entity = create_entity( &review )?;

    // Author (Agent) anchor
    let (base, base_hash) = devhub_types::create_path( &crate::agent_path_base( None ), vec![ ANCHOR_REVIEWS ] );
    debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
    entity.link_from( &base_hash, LT_NONE, TAG_REVIEW.into() )?;

    for subject_id in input.subject_ids {
	let (base, base_hash) = devhub_types::create_path( ANCHOR_REVIEWS, vec![ subject_id.to_owned() ] );
	debug!("Linking agent ({}) to ENTRY: {}", fmt_path( &base ), entity.id );
	entity.link_from( &base_hash, LT_NONE, TAG_REVIEW.into() )?;
    }

    Ok( entity )
}




pub fn get_review(input: GetEntityInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Get Review: {}", input.id );
    let entity = get_entity::<ReviewEntry>( &input.id )?;

    Ok( entity )
}




#[derive(Debug, Deserialize, Clone)]
pub struct ReviewUpdateOptions {
    pub accuracy_rating: Option<u8>,
    pub efficiency_rating: Option<u8>,
    pub message: Option<String>,
    pub published_at: Option<u64>,
    pub last_updated: Option<u64>,
    pub metadata: Option<BTreeMap<String, serde_yaml::Value>>,
}
pub type ReviewUpdateInput = UpdateEntityInput<ReviewUpdateOptions>;

pub fn update_review(input: ReviewUpdateInput) -> AppResult<Entity<ReviewEntry>> {
    debug!("Updating Review: {}", input.addr );
    let props = input.properties.clone();

    let entity = update_entity(
	&input.addr,
	|mut current : ReviewEntry, _| {
	    current.accuracy_rating = props.accuracy_rating
		.unwrap_or( current.accuracy_rating );
	    current.efficiency_rating = props.efficiency_rating
		.unwrap_or( current.efficiency_rating );
	    current.message = props.message
		.unwrap_or( current.message );
	    current.published_at = props.published_at
		.unwrap_or( current.published_at );
	    current.last_updated = props.last_updated
		.unwrap_or( current.last_updated );
	    current.metadata = props.metadata
		.unwrap_or( current.metadata );

	    Ok( current )
	})?;

    Ok( entity )
}




pub fn delete_review(addr: EntryHash) -> AppResult<Entity<ReviewEntry>> {
    debug!("Delete Review Version: {}", addr );
    let entity = update_entity(
	&addr,
	|mut current : ReviewEntry, _| {
	    current.deleted = true;

	    Ok( current )
	})?;

    Ok( entity )
}
