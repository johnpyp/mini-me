use crate::db::DbConn;

#[derive(sqlx::FromRow)]
pub struct DynamicCommand {
    pub id: String,
    pub guild_id: String,
    pub command: String,
    pub response: String,
}

impl DynamicCommand {
    pub async fn delete_command(
        conn: &DbConn,
        guild_id: &str,
        target_command: &str,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query!(
            r#"
        DELETE FROM dynamic_commands
        WHERE guild_id = $1 AND command = $2;
            "#,
            guild_id,
            target_command
        )
        .execute(conn)
        .await?;
        return Ok(true);
    }

    pub async fn get_command(
        conn: &DbConn,
        guild_id: &str,
        target_command: &str,
    ) -> Result<Option<DynamicCommand>, sqlx::Error> {
        let command = sqlx::query_as!(
            DynamicCommand,
            r#"
        SELECT * FROM dynamic_commands
        WHERE guild_id = $1 AND command = $2;
            "#,
            guild_id,
            target_command
        )
        .fetch_optional(conn)
        .await?;
        return Ok(command);
    }

    pub async fn add(&self, conn: &DbConn) -> Result<bool, sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO dynamic_commands (id, command, response, guild_id)
        VALUES ($1, $2, $3, $4);
            "#,
            self.id,
            self.command,
            self.response,
            self.guild_id
        )
        .execute(conn)
        .await?;
        return Ok(true);
    }
}

#[derive(sqlx::FromRow)]
pub struct GuildData {
    pub guild_id: String,
    pub moderator_role_id: Option<String>,
    pub dynamic_prefix: Option<String>,
}

impl GuildData {
    pub async fn upsert(&self, conn: &DbConn) -> Result<bool, sqlx::Error> {
        sqlx::query!(
            r#"
        INSERT INTO guild_data (guild_id, moderator_role_id, dynamic_prefix)
        VALUES ($1, $2, $3)
        ON CONFLICT (guild_id)
        DO UPDATE SET moderator_role_id = $2, dynamic_prefix = $3;
            "#,
            self.guild_id,
            self.moderator_role_id,
            self.dynamic_prefix
        )
        .execute(conn)
        .await?;
        return Ok(true);
    }

    pub async fn get(conn: &DbConn, guild_id: &str) -> Result<Option<GuildData>, sqlx::Error> {
        let guild_data = sqlx::query_as!(
            GuildData,
            r#"
        SELECT * FROM guild_data
        WHERE guild_id = $1;
            "#,
            guild_id,
        )
        .fetch_optional(conn)
        .await?;
        return Ok(guild_data);
    }
}

impl Default for GuildData {
    fn default() -> Self {
        GuildData {
            guild_id: String::from(""),
            moderator_role_id: None,
            dynamic_prefix: None,
        }
    }
}
