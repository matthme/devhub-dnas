
use hdk::prelude::*;

use crate::utils;
use crate::errors::{ RuntimeError };


#[derive(Debug, Serialize, Deserialize)]
pub struct EntityInfo {
    pub name: String,

    // optional
    pub website: Option<String>,
}

#[hdk_entry(id = "dna", visibility="public")]
pub struct DnaEntry {
    pub name: String,
    pub description: String,
    pub published_at: u64,

    // optional
    pub developer: Option<EntityInfo>,
    pub deprecation: Option<DeprecationNotice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecationNotice {
    pub message: String,

    // optional
    pub recommended_alternatives: Option<HeaderHash>,
}

impl DeprecationNotice {
    pub fn new(message: String) -> Self {
	Self {
	    message: message,
	    recommended_alternatives: None,
	}
    }
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSummary {
    pub name: String,
    pub description: String,
    pub published_at: u64,

    // optional
    pub developer: Option<String>,
    pub deprecation: Option<bool>,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaInfo {
    pub name: String,
    pub description: String,
    pub published_at: u64,

    // optional
    pub developer: Option<EntityInfo>,
    pub deprecation: Option<DeprecationNotice>,
}

impl DnaEntry {
    pub fn to_info(self) -> DnaInfo {
	self.into()
    }

    pub fn to_summary(self) -> DnaSummary {
	self.into()
    }
}

impl TryFrom<Element> for DnaEntry {
    type Error = WasmError;
    fn try_from(element: Element) -> Result<Self, Self::Error> {
	element.entry()
	    .to_app_option::<Self>()?
	    .ok_or(WasmError::from(RuntimeError::DeserializationError(element)))
    }
}

impl From<DnaEntry> for DnaInfo {
    fn from(dna: DnaEntry) -> Self {
	DnaInfo {
	    name: dna.name,
	    description: dna.description,
	    published_at: dna.published_at,
	    developer: dna.developer,
	    deprecation: dna.deprecation,
	}
    }
}

impl From<DnaEntry> for DnaSummary {
    fn from(dna: DnaEntry) -> Self {
	DnaSummary {
	    name: dna.name,
	    description: dna.description,
	    published_at: dna.published_at,
	    developer: match dna.developer {
		Some(dev) => Some(dev.name),
		None => None,
	    },
	    deprecation: match dna.deprecation {
		Some(_) => Some(true),
		None => None,
	    },
	}
    }
}





#[hdk_entry(id = "dna_version", visibility="public")]
pub struct DnaVersionEntry {
    pub for_dna: EntryHash,
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
    pub contributors: Vec<String>,
    pub changelog: String,
    pub chunk_addresses: Vec<EntryHash>,
}

// Summary
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionSummary {
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
}

// Full
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaVersionInfo {
    pub for_dna: Option<DnaSummary>,
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
    pub contributors: Vec<String>,
    pub changelog: String,
    pub chunk_addresses: Vec<EntryHash>,
}

// Package
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaPackage {
    pub for_dna: DnaSummary,
    pub version: u64,
    pub published_at: u64,
    pub file_size: u64,
    pub bytes: SerializedBytes,
    pub contributors: Vec<String>,
    pub changelog: String,
}

impl DnaVersionEntry {
    pub fn to_info(self) -> DnaVersionInfo {
	self.into()
    }

    pub fn to_summary(self) -> DnaVersionSummary {
	self.into()
    }
}

impl TryFrom<Element> for DnaVersionEntry {
    type Error = WasmError;
    fn try_from(element: Element) -> Result<Self, Self::Error> {
	element.entry()
	    .to_app_option::<Self>()?
	    .ok_or(WasmError::from(RuntimeError::DeserializationError(element)))
    }
}

impl From<DnaVersionEntry> for DnaVersionInfo {
    fn from(version: DnaVersionEntry) -> Self {
	let mut dna_summary : Option<DnaSummary> = None;

	if let Some((_,element)) = utils::fetch_entry_latest( version.for_dna ).ok() {
	    dna_summary = match DnaEntry::try_from( element ) {
		Ok(dna) => Some(dna.to_summary()),
		Err(_) => None,
	    };
	};

	DnaVersionInfo {
	    for_dna: dna_summary,
	    version: version.version,
	    published_at: version.published_at,
	    file_size: version.file_size,
	    contributors: version.contributors,
	    changelog: version.changelog,
	    chunk_addresses: version.chunk_addresses,
	}
    }
}

impl From<DnaVersionEntry> for DnaVersionSummary {
    fn from(version: DnaVersionEntry) -> Self {
	DnaVersionSummary {
	    version: version.version,
	    published_at: version.published_at,
	    file_size: version.file_size,
	}
    }
}





#[hdk_entry(id = "dna_chunk", visibility="public")]
pub struct DnaChunkEntry {
    pub sequence: SequencePosition,
    pub bytes: SerializedBytes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequencePosition {
    pub position: u64,
    pub length: u64,
}




#[hdk_extern]
fn validate_create_entry_dna(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    if let Ok(_dna) = DnaEntry::try_from( validate_data.element ) {
	return Ok(ValidateCallbackResult::Valid);
    }

    Ok(ValidateCallbackResult::Invalid("DNA entry is not right".to_string()))
}




#[cfg(test)]
pub mod tests {
    use super::*;

    fn create_dnaentry() -> crate::DnaEntry {
	crate::DnaEntry {
	    name: String::from("Game Turns"),
	    description: String::from("A tool for turn-based games to track the order of player actions"),
	    published_at: 1618855430,

	    // optional
	    developer: Some(EntityInfo {
		name: String::from("Open Games Collective"),
		website: Some(String::from("https://github.com/open-games-collective/")),
	    }),
	    deprecation: None,
	}
    }

    #[test]
    ///
    fn dna_to_summary_test() {
	let dna1 = create_dnaentry();
	let dna2 = create_dnaentry();

	assert_eq!(dna1.name, "Game Turns");

	let dna_info = dna1.to_info();

	assert_eq!(dna_info.name, "Game Turns");

	let dna_summary = dna2.to_summary();

	assert_eq!(dna_summary.name, "Game Turns");
    }
}
