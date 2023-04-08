// #[derive(Serialize, Deserialize)]
// pub struct ThreadCardData {
//     pub title: String,
//     pub created_at: NaiveDateTime,
//     pub creator_name: String,
//     pub creator_avatar_url: Option<String>,
//     pub post_count: Option<i64>,
//     pub last_post_created_at: Option<NaiveDateTime>,
//     pub last_post_creator_name: Option<String>,
//     pub last_post_creator_avatar_url: Option<String>,
// }

// #[server(GetForumPageData, "/api")]
// pub async fn get_forum_page_data(cx: Scope, slug: String, id: String) -> Result<(), ServerFnError> {
//     use crate::database::get_db;
//     let pool = get_db(cx).await?;

//     let id = id
//         .parse::<i32>()
//         .map_err(|_| ServerFnError::ServerError("Invalid path".to_string()))?;

//     let data = sqlx::query_as!(
//         ThreadCardData,
//         r#"
//     SELECT thread.title,
//         thread.created_at,
//         forum_user.username AS creator_name,
//         forum_user.avatar_url
//         AS creator_avatar_url,
//         COUNT(post.id) AS post_count,
//         MAX(post.created_at) AS last_post_created_at,
//         MAX(forum_user2.username) AS last_post_creator_name,
//         MAX(forum_user2.avatar_url) AS last_post_creator_avatar_url
//     FROM thread
//     INNER JOIN forum_user ON thread.creator_id = forum_user.id
//     LEFT JOIN post ON thread.id = post.thread_id
//     LEFT JOIN forum_user AS forum_user2 ON post.creator_id = forum_user2.id
//     GROUP BY thread.id, forum_user.username, forum_user.avatar_url
//     ORDER BY last_post_created_at DESC
//     "#
//     )
//     .fetch_all(&pool)
//     .await;
//     Ok(())
// }

// let data = create_resource(
//     cx,
//     || (),
//     move |_| async move {
//         let route = use_route(cx);
//         let path = route.path();
//         let path = path.replace("/forum/", "");
//         let (slug, id) = match path.split('.').next_tuple() {
//             Some((slug, id)) => {
//                 if slug.is_empty() || id.is_empty() {
//                     return server_error!("Invalid path");
//                 }
//                 (slug, id)
//             }
//             None => {
//                 return server_error!("Invalid path");
//             }
//         };
//         get_forum_page_data(cx, slug.to_string(), id.to_string()).await
//     },
// );
