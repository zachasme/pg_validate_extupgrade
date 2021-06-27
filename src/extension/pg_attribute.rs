/*----------------------------------------------------------------------------
 * Author: Julien Rouhaud
 * Copyright: Copyright (c) 2021 : Julien Rouhaud - All rights reserved
 *---------------------------------------------------------------------------*/
use postgres::{Row, Transaction};

use crate::{
	compare::*,
	DbStruct,
	pgtype::*,
};

DbStruct! {
	Attribute:attname:Attribute {
		attname: Name,
		atttype: Text = ("pg_catalog.format_type(a.atttypid, a.atttypmod)"),
		attstattarget: Integer,
		attnum: Smallint,
		attndims: Integer,
		attstorage: Char,
		attcompression: Char {PG_14},
		attnotnull: Bool,
		attdefault: Option<Text> = ("pg_get_expr(d.adbin, d.adrelid, true)"),
		attidentity: Char {PG_10},
		attgenerated: Char {PG_12},
		attislocal: Bool,
		attinhcount: Integer,
		attcollation: Option<Name> = ("c.collname"),
		attacl: Option<Text> = ("attacl::text"),
		attoptions: Option<Vec<Text>>,
		attfdwoptions: Option<Vec<Text>>,
	}
}

impl Attribute {
	pub fn snapshot(client: &mut Transaction, relid: u32, pgver: u32)
		-> Vec<Attribute>
	{
		let mut atts = Vec::new();

		let sql = format!("SELECT {} \
			FROM pg_attribute a \
			JOIN pg_type t ON t.oid = a.atttypid \
			LEFT JOIN pg_collation c ON c.oid = a.attcollation \
				AND a.attcollation <> t.typcollation \
			LEFT JOIN pg_attrdef d ON d.adrelid = a.attrelid \
				AND d.adnum = a.attnum AND a.atthasdef \
			WHERE attnum > 0 \
			AND NOT attisdropped \
			AND attrelid = $1 \
			ORDER BY attnum",
			Attribute::tlist(pgver).join(", "),
		);

		let rows = client.query(&sql[..], &[&relid])
			.expect("Could net get pg_attribute rows");

		for row in &rows {
			atts.push(Attribute::from_row(row));
		};

		atts
	}
}
