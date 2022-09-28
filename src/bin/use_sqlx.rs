// https://github.com/launchbadge/sqlx/blob/main/examples/sqlite/todos/src/main.rs

use bincode::deserialize_from;
use sqlx::sqlite::{
	SqliteConnectOptions,
	// SqlitePool,
	SqlitePoolOptions,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// https://kerkour.com/high-performance-rust-with-sqlite
	let connection_options =
		SqliteConnectOptions::from_str("./data/data.db")?.create_if_missing(true);
	// .journal_mode(SqliteJournalMode::Wal)
	// .synchronous(SqliteSynchronous::Normal)
	// .busy_timeout(pool_timeout);

	let pool = SqlitePoolOptions::new()
		// .max_connections(pool_max_connections)
		// .connect_timeout(pool_timeout)
		.connect_with(connection_options)
		.await?;

	// sqlx::migrate!("./migrations").run(&pool).await?;

	// let pool = SqlitePool::connect("./data/data.db").await?;

	// println!("oiee");

	// let conn = pool.acquire().await?;

	sqlx::query("drop table if exists simplified").execute(&pool).await?;
	sqlx::query("drop table if exists traditional").execute(&pool).await?;

	sqlx::query(
		"create table simplified (
			id integer primary key autoincrement,
			character text not null
	  )",
	)
	.execute(&pool)
	.await?;
	sqlx::query(
		"create table traditional (
			id integer primary key autoincrement,
			character text not null
	  )",
	)
	.execute(&pool)
	.await?;

	let input = Path::new("data/s2t.profile");
	let rdr = File::open(input).map(BufReader::new)?;
	let dictionary: HashMap<String, String> = deserialize_from(rdr)?;

	// println!("{}", dictionary.iter().take(5).len());

	for r in dictionary.iter() {
		let (simplified, traditional) = r;
		// println!("{:?}", r);
		// println!("{simplified}");

		// Insert the simplified character, then obtain the ID of this row
		let simplified_id = sqlx::query("insert into simplified (character) values (?1)")
			.bind(simplified)
			.execute(&pool)
			.await?
			.last_insert_rowid();
		// Insert the traditional character, then obtain the ID of this row
		let traditional_id = sqlx::query("insert into traditional (character) values (?1)")
			.bind(traditional)
			.execute(&pool)
			.await?
			.last_insert_rowid();

		// println!("simplified_id: {}", simplified_id);
		// println!("traditional_id: {}", traditional_id);
	}

	Ok(())
}
