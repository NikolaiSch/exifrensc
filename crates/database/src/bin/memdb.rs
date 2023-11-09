use database::connection::DB;
use entity::exif;
use entity::file;
use entity::prelude::*;
use sea_orm::prelude::*;
use sea_orm::sea_query::InsertStatement;
use sea_orm::sea_query::Query;
use sea_orm::Set;

#[tokio::main]
async fn main() {
    let db = DB::new_memory_db().await.unwrap();

    //     pub struct Model {
    //     #[sea_orm(primary_key)]
    //     pub id: i32,
    //     pub path: String,
    //     pub created: i64,
    //     pub modified: i64,
    //     pub orig_file_name: String,
    //     pub new_file_name: String,
    //     pub nksc_path: String,
    //     pub in_nx_studio: bool,
    //     pub tmp_lock: bool,
    //     pub locked: bool,
    // }

    // give me a testcase

    let a = file::ActiveModel {
        id: Set(1),
        path: Set("path".to_owned()),
        created: Set(1),
        modified: Set(1),
        orig_file_name: Set("orig_file_name".to_owned()),
        new_file_name: Set("new_file_name".to_owned()),
        nksc_path: Set("nksc_path".to_owned()),
        in_nx_studio: Set(true),
        tmp_lock: Set(true),
        locked: Set(true),
    };

    a.insert(&db).await.unwrap();

    //     pub struct Model {
    //     #[sea_orm(primary_key)]
    //     pub id: i32,
    //     pub path: String,
    //     pub tag: String,
    //     pub tag_id: i64,
    //     pub value: String,
    // }

    let x = exif::ActiveModel {
        id: Set(1),
        path: Set("path".to_owned()),
        tag: Set("tag".to_owned()),
        value: Set("value".to_owned()),
        tag_id: Set(1),
        file_id: Set(1),
    };

    x.insert(&db).await.unwrap();

    let x = exif::ActiveModel {
        id: Set(2),
        path: Set("2".to_owned()),
        tag: Set("3".to_owned()),
        value: Set("4".to_owned()),
        tag_id: Set(1),
        file_id: Set(1),
    };

    x.insert(&db).await.unwrap();

    let e = File::find_by_id(1).one(&db).await.unwrap().unwrap();

    println!("file: {:?}", e);

    e.find_related(Exif)
        .all(&db)
        .await
        .unwrap()
        .iter()
        .for_each(|e| {
            println!("exif: {:?}", e);
        });
}
