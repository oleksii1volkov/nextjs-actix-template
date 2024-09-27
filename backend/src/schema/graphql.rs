use actix_web::web::Data;
use async_graphql::{Context, FieldResult, Object, OutputType, Schema, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::{error, info};

use crate::config::ApplicationData;

#[derive(SimpleObject, Serialize, Deserialize, FromRow, Clone)]
pub struct Tour {
    id: i32,
    title: String,
    description: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    price: Option<f64>,
    rating: Option<f64>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    location: Option<String>,
    image_url: Option<String>,
    is_active: bool,
    max_participants: Option<i32>,
}

pub struct QueryRoot;

fn get_database_pool<'a>(context: &Context<'a>) -> &'a sqlx::PgPool {
    &context
        .data::<Data<ApplicationData>>()
        .unwrap()
        .database_pool
}

#[Object]
impl QueryRoot {
    async fn get_tours(&self, context: &Context<'_>) -> FieldResult<Vec<Tour>> {
        match sqlx::query_as::<_, Tour>(
            r#"
            SELECT * FROM public."Tours"
            "#,
        )
        .fetch_all(get_database_pool(context))
        .await
        {
            Ok(tours) => Ok(tours),
            Err(error) => {
                error!("Failed to get tours: {}", error);
                Err(async_graphql::Error::new(format!(
                    "Failed to get tours: {}",
                    error
                )))
            }
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_tour(
        &self,
        context: &Context<'_>,
        title: String,
        description: Option<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        price: Option<f64>,
        rating: Option<f64>,
        location: Option<String>,
        image_url: Option<String>,
        is_active: bool,
        max_participants: Option<i32>,
    ) -> FieldResult<Tour> {
        let now = Utc::now().naive_utc();

        match sqlx::query_as::<_, Tour>(
            r#"
            INSERT INTO tours (title, description, start_date, end_date, price, rating, location, image_url, is_active, max_participants, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#
        )
        .bind(title)
        .bind(description)
        .bind(start_date)
        .bind(end_date)
        .bind(price)
        .bind(rating)
        .bind(location)
        .bind(image_url)
        .bind(is_active)
        .bind(max_participants)
        .bind(now)
        .bind(now)
        .fetch_one(get_database_pool(context))
        .await {
            Ok(tour) => Ok(tour),
            Err(error) => {
                error!("Failed to create tour: {}", error);
                Err(async_graphql::Error::new(format!(
                    "Failed to create tour: {}",
                    error
                )))
            }
        }
    }

    async fn update_tour(
        &self,
        context: &Context<'_>,
        id: i32,
        title: Option<String>,
        description: Option<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        price: Option<f64>,
        rating: Option<f64>,
        location: Option<String>,
        image_url: Option<String>,
        is_active: Option<bool>,
        max_participants: Option<i32>,
    ) -> FieldResult<Tour> {
        let now = Utc::now().naive_utc();

        match sqlx::query_as::<_, Tour>(
            r#"
            UPDATE tours
            SET title = COALESCE($2, title),
                description = COALESCE($3, description),
                start_date = COALESCE($4, start_date),
                end_date = COALESCE($5, end_date),
                price = COALESCE($6, price),
                rating = COALESCE($7, rating),
                location = COALESCE($8, location),
                image_url = COALESCE($9, image_url),
                is_active = COALESCE($10, is_active),
                max_participants = COALESCE($11, max_participants),
                updated_at = $12
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(title)
        .bind(description)
        .bind(start_date)
        .bind(end_date)
        .bind(price)
        .bind(rating)
        .bind(location)
        .bind(image_url)
        .bind(is_active)
        .bind(max_participants)
        .bind(now)
        .fetch_one(get_database_pool(context))
        .await
        {
            Ok(tour) => Ok(tour),
            Err(error) => {
                error!("Failed to update tour: {}", error);
                Err(async_graphql::Error::new(format!(
                    "Failed to update tour: {}",
                    error
                )))
            }
        }
    }
}

pub type ApplicationSchema = Schema<QueryRoot, MutationRoot, async_graphql::EmptySubscription>;
