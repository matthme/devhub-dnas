use devhub_types::{
    DevHubResponse, EntityResponse, GetEntityInput,
    constants::{ VALUE_MD, ENTITY_MD },
    web_asset_entry_types::{
	FileEntry,
	FilePackage,
    },
    composition,
    catch,
};
use hdk::prelude::*;

mod files;
mod constants;



entry_defs![
    PathEntry::entry_def(),
    FileEntry::entry_def()
];


pub fn root_path(pubkey: Option<AgentPubKey>) -> ExternResult<Path> {
    let pubkey = pubkey
	.unwrap_or( agent_info()?.agent_initial_pubkey );
    let path = Path::from( format!("{:?}", pubkey ) );

    debug!("Agent ({:?}) root path is: {:?}", pubkey, path.path_entry_hash()? );
    Ok( path )
}
pub fn root_path_hash(pubkey: Option<AgentPubKey>) -> ExternResult<EntryHash> {
    Ok( root_path( pubkey )?.path_entry_hash()? )
}


#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let agent = agent_info()?.agent_initial_pubkey;
    let path = root_path( Some(agent.to_owned()) )?;

    debug!("Ensure the agent ({:?}) root path is there: {:?}", agent, path.path_entry_hash()? );
    path.ensure()?;

    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn whoami(_: ()) -> ExternResult<DevHubResponse<AgentInfo>> {
    Ok(composition( agent_info()?, VALUE_MD ))
}


// Files
#[hdk_extern]
fn create_file(input: files::CreateInput) -> ExternResult<EntityResponse<FileEntry>> {
    let entity = catch!( files::create_file( input ) );

    Ok(composition( entity, ENTITY_MD ))
}

#[hdk_extern]
fn get_file(input: GetEntityInput) -> ExternResult<EntityResponse<FilePackage>> {
    let entity = catch!( files::get_file( input ) );

    Ok(composition( entity, ENTITY_MD ))
}
