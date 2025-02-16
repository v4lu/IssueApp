use crate::models::docs::{Doc, DocPagable, DocPageRequest, DocRequest, UpdateDocRequest};

pub struct DocsRepository {
    pool: sqlx::PgPool,
}

impl DocsRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(
        &self,
        org_id: uuid::Uuid,
        creator_id: uuid::Uuid,
        request: DocRequest,
    ) -> Result<Doc, sqlx::Error> {
        let doc = sqlx::query_as!(
            Doc,
            r#"
            INSERT INTO docs (org_id, creator_id, title, content, number)
            VALUES ($1, $2, $3, $4, (SELECT COALESCE(MAX(number), 0) + 1 FROM docs WHERE org_id = $1))
            RETURNING id, org_id, creator_id, title, content, created_at as "created_at!: chrono::DateTime<chrono::Utc>",
            updated_at as "updated_at!: chrono::DateTime<chrono::Utc>", sharable, sharable_link, number
            "#,
            org_id,
            creator_id,
            request.title,
            request.content
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(doc)
    }

    pub async fn get(&self, org_id: uuid::Uuid, doc_id: uuid::Uuid) -> Result<Doc, sqlx::Error> {
        let doc = sqlx::query_as!(
            Doc,
            r#"
            SELECT id, org_id, creator_id, title, content, created_at as "created_at!: chrono::DateTime<chrono::Utc>",
            updated_at as "updated_at!: chrono::DateTime<chrono::Utc>", sharable, sharable_link, number
            FROM docs
            WHERE org_id = $1 AND id = $2
            "#,
            org_id,
            doc_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(doc)
    }

    pub async fn update(&self, org_id: uuid::Uuid,  doc_id: uuid::Uuid, request: UpdateDocRequest) -> Result<Doc, sqlx::Error> {
        let doc = sqlx::query_as!(
            Doc,
            r#"
            UPDATE docs
            SET title = $3, content = $4, updated_at = now()
            WHERE org_id = $1 AND id = $2
            RETURNING id, org_id, creator_id, title, content, created_at as "created_at!: chrono::DateTime<chrono::Utc>",
            updated_at as "updated_at!: chrono::DateTime<chrono::Utc>", sharable, sharable_link, number
            "#,
            org_id,
       
            doc_id,
            request.title,
            request.content
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(doc)
    }

    pub async fn delete(&self, org_id: uuid::Uuid, doc_id: uuid::Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM docs
            WHERE org_id = $1 AND id = $2
            "#,
            org_id,
            doc_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }


    pub async fn get_all(&self, org_id: uuid::Uuid, request: DocPageRequest) -> Result<DocPagable, sqlx::Error> {
       let ext_search = format!(
            r#"
            AND title ILIKE '%{}%'
            "#,
            request.search.clone().unwrap_or("".to_string())
        ); 

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as "count!" FROM docs
            WHERE org_id = $1
            "#,
            org_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;



        let query = format!(
            r#"
            SELECT id, org_id, creator_id, title, content, created_at,
            updated_at, sharable, sharable_link, number
            FROM docs
            WHERE org_id = $1
            {}
            ORDER BY {} {}
            LIMIT $2 OFFSET $3
            "#,
            ext_search,
            request.get_sort(),
            request.get_order()
        );

             let docs = sqlx::query_as::<_, Doc>(&query)
            .bind(org_id)
            .bind(request.page_size)
            .bind(request.page * request.page_size)
            .fetch_all(&self.pool)
            .await?;

        Ok(DocPagable {
            docs,
            total,
            page: request.page,
            page_size: request.page_size,
            last_page: (total as f64 / request.page_size as f64).ceil() as i64,
        })
    }

}
