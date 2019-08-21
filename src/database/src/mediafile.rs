use crate::library::Library;
use crate::media::Media;
use crate::schema::mediafile;
use diesel::prelude::*;
use diesel::debug_query;

#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug, Associations)]
#[belongs_to(Library, foreign_key = "library_id")]
#[belongs_to(Media, foreign_key = "media_id")]
#[table_name = "mediafile"]
pub struct MediaFile {
    pub id: i32,
    pub media_id: Option<i32>,
    pub library_id: i32,
    pub target_file: String,
    
    pub raw_name: String,
    pub raw_year: Option<i32>,

    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "mediafile"]
pub struct InsertableMediaFile {
    pub media_id: Option<i32>,
    pub library_id: i32,
    pub target_file: String,
    
    pub raw_name: String,
    pub raw_year: Option<i32>,

    pub quality: Option<String>,
    pub codec: Option<String>,
    pub container: Option<String>,
    pub audio: Option<String>,
    pub original_resolution: Option<String>,
    pub duration: Option<i32>,
}

#[derive(AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name = "mediafile"]
pub struct UpdateMediaFile {
    pub media_id: Option<i32>,
    pub target_file: Option<String>,
    pub raw_name: Option<String>,
    pub raw_year: Option<Option<i32>>,
    pub quality: Option<Option<String>>,
    pub codec: Option<Option<String>>,
    pub container: Option<Option<String>>,
    pub audio: Option<Option<String>>,
    pub original_resolution: Option<Option<String>>,
    pub duration: Option<Option<i32>>,
}

impl MediaFile {
    pub fn get_by_lib(
        conn: &diesel::PgConnection,
        lib: &Library
    ) -> Result<Vec<Self>, diesel::result::Error> {
        Self::belonging_to(lib)
            .load::<Self>(conn)
    }
}

impl InsertableMediaFile {
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;
        let result: i32 = diesel::insert_into(mediafile)
            .values(self)
            .returning(id)
            .get_result(conn)?;

        Ok(result)
    }
}

impl UpdateMediaFile {
    pub fn update(
        &self,
        conn: &diesel::PgConnection,
        _id: i32
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::mediafile::dsl::*;
        let entry = mediafile
            .filter(id.eq(_id));

        let q = diesel::update(entry).set(self);
        /*
        let query = diesel::debug_query::<diesel::pg::Pg, _>(
            &q
        ).to_string();
        println!("Q: {}", query);
        */
        q.execute(conn)
    }
}