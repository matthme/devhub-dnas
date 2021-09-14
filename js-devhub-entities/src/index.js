
const { AgentClient,
	HoloHashTypes,
	...HolochainClient }		= require('@whi/holochain-client');
const { Translator }			= require('@whi/essence');
const EntityArchitectLib		= require('@whi/entity-architect');

const { Architecture,
	EntityType,
	EntryHash,
	AgentPubKey }			= EntityArchitectLib;


let debug				= false;
function log ( msg, ...args ) {
    let datetime			= (new Date()).toISOString();
    console.log(`${datetime} [ src/index. ]  INFO: ${msg}`, ...args );
}



//
// hApps Entity Types
//
const Happ				= new EntityType("happ");

Happ.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.designer		= new AgentPubKey( content.designer );

    if ( content.gui ) {
	content.gui.asset_group_id	= new EntryHash( content.gui.asset_group_id );
    }

    return content;
});

Happ.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.designer		= new AgentPubKey( content.designer );

    return content;
});

const HappRelease			= new EntityType("happ_release");

HappRelease.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.for_happ		= Schema.deconstruct( "entity", content.for_happ );

    for (let k in content.resources ) {
	content.resources[k]	= new EntryHash( content.resources[k] );
    }

    return content;
});

HappRelease.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.for_happ		= new EntryHash( content.for_happ );

    return content;
});


//
// DNA Repository Entity Types
//

// Profiles
const Profile				= new EntityType("profile");

Profile.model("info");


// DNAs
const Dna				= new EntityType("dna");

Dna.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.developer.pubkey	= new AgentPubKey( content.developer.pubkey );

    return content;
});
Dna.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.developer		= new AgentPubKey( content.developer );

    return content;
});


const DnaVersion			= new EntityType("dna_version");

DnaVersion.model("package", function ( content ) {
    content.for_dna		= Schema.deconstruct( "entity", content.for_dna );
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.bytes		= new Uint8Array(content.bytes);

    content.contributors.forEach( ([email, pubkey], i) => {
	content.contributors[i]		= {
	    email,
	    "agent": pubkey === null ? null : new AgentPubKey(pubkey),
	};
    });

    return content;
});
DnaVersion.model("info", function ( content ) {
    content.for_dna		= Schema.deconstruct( "entity", content.for_dna );
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );

    content.contributors.forEach( ([email, pubkey], i) => {
	content.contributors[i]		= {
	    email,
	    "agent": pubkey === null ? null : new AgentPubKey(pubkey),
	};
    });

    return content;
});
DnaVersion.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );

    return content;
});


// Zomes
const Zome				= new EntityType("zome");

Zome.model("info", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.developer.pubkey	= new AgentPubKey( content.developer.pubkey );

    return content;
});
Zome.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.developer		= new AgentPubKey( content.developer );

    return content;
});


const ZomeVersion			= new EntityType("zome_version");

ZomeVersion.model("info", function ( content ) {
    content.for_zome		= Schema.deconstruct( "entity", content.for_zome );
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.mere_memory_addr	= new EntryHash(content.mere_memory_addr);

    return content;
});
ZomeVersion.model("summary", function ( content ) {
    content.published_at	= new Date( content.published_at );
    content.last_updated	= new Date( content.last_updated );
    content.mere_memory_addr	= new EntryHash(content.mere_memory_addr);

    return content;
});


//
// Web Asset Entity Types
//
const File				= new EntityType("file");

File.model("info", function ( content ) {
    content.author		= new AgentPubKey( content.author );
    content.published_at	= new Date( content.published_at );

    content.chunk_addresses.forEach( (addr, i) => {
	content.chunk_addresses[i]	= new EntryHash(addr);
    });

    return content;
});

const FileChunk				= new EntityType("file_chunk");

FileChunk.model("info");


//
// Grouping Entity Definitions
//
const Schema				= new Architecture([
    Happ, HappRelease,
    Profile, Dna, DnaVersion, Zome, ZomeVersion,
    File, FileChunk,
]);


const Interpreter			= new Translator(["AppError", "UtilsError", "DNAError", "UserError", "WasmError"], {
    "rm_stack_lines": 2,
});

const CLIENT_DEFAULT_OPTIONS		= {
    "timeout": 5_000,
    "parse_essence": true,
    "parse_entities": true,
    "simulate_latency": false,
};

class Client {
    constructor ( agent_pubkey, dnas, address, options = {} ) {
	this.agent_pubkey		= agent_pubkey;
	this.dnas			= dnas;
	this.cell_id			= [ this.dna_hash, this.agent_pubkey ];
	this.options			= Object.assign( {}, CLIENT_DEFAULT_OPTIONS, options );
	this._client			= new AgentClient( this.agent_pubkey, this.dnas, address );
    }

    async destroy () {
	this._client.close();
    }

    async call ( dna_nickname, zome_name, fn_name, args = null, opts_override = {} ) {
	let args_debug;
	const opts			= Object.assign( {}, this.options, opts_override );

	if ( opts.simulate_latency )
	    await new Promise( f => setTimeout(f, (Math.random() * 1_000) + 500) ); // range 500ms to 1500ms

	if ( debug ) {
	    if ( args === null )
		args_debug		= " null ";
	    else if ( args === undefined )
		args_debug		= ` ${typeof args} `;
	    else if ( args.constructor.name === "Object" )
		args_debug		= `{ ${Object.keys(args || {}).join(", ")} }`;
	    else
		args_debug		= ` ${args.constructor.name} `;
	}
	debug && log("Calling conductor: %s->%s(%s)", zome_name, fn_name, args_debug );

	let response;
	try {
	    response			= await this._client.call(
		dna_nickname, zome_name, fn_name, args, opts.timeout
	    );
	    debug && log("Received response for: %s->%s(%s)", zome_name, fn_name, args_debug );
	    debug && log("Full response:", response );
	} catch ( err ) {
	    debug && log("Conductor returned error: %s", err );
	    if ( err instanceof Error )
		console.error( err );

	    if ( err.type === "error" ) { // Holochain error
		if ( err.data.data.length > 2000 )
		    err.data.data	=  err.data.data.slice(0,1999) + "\u2026";
	    }

	    throw err;
	}

	if ( opts.parse_essence === false )
	    return response;

	let pack;
	try {
	    pack			= Interpreter.parse( response );
	} catch ( err ) {
	    debug && log("Failed to interpret Essence package: %s", String(err) );
	    console.error( err.stack );
	    throw err;
	}

	let payload			= pack.value();

	if ( payload instanceof Error ) {
	    debug && log("Throwing error package: %s::%s( %s )", payload.kind, payload.name, payload.message );
	    throw payload;
	}

	if ( opts.parse_entities === false )
	    return payload;

	let composition			= pack.metadata('composition');

	if ( composition === undefined )
	    return payload;

	try {
	    return Schema.deconstruct( composition, payload );
	} catch ( err ) {
	    debug && log("Failed to deconstruct payload: %s", String(err) );
	    console.error( err.stack );
	    throw err;
	}
    };
}


module.exports = {
    Client,
    Schema,

    Happ,
    HappRelease,
    Profile,
    Dna,
    DnaVersion,
    Zome,
    ZomeVersion,
    File,
    FileChunk,

    "EntityArchitect": EntityArchitectLib,
    "HoloHashes": HoloHashTypes,

    HolochainClient,

    logging () {
	debug				= true;
    },
};
