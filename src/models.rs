use crate::db::DbConn;

#[derive(sqlx::FromRow)]
pub struct DynamicCommand {
    pub id: String,
    pub guild_id: String,
    pub attachment_urls: Option<Vec<String>>,
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
        Ok(true)
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
        Ok(command)
    }

    pub async fn get_commands_by_guild(
        conn: &DbConn,
        guild_id: &str,
    ) -> Result<Vec<DynamicCommand>, sqlx::Error> {
        let commands = sqlx::query_as!(
            DynamicCommand,
            r#"
        SELECT * FROM dynamic_commands
        WHERE guild_id = $1;
            "#,
            guild_id
        )
        .fetch_all(conn)
        .await?;
        Ok(commands)
    }

    pub async fn add(&self, conn: &DbConn) -> Result<bool, sqlx::Error> {
        let attachment_urls: Option<&[String]> = if let Some(urls) = &self.attachment_urls {
            Some(urls.as_slice())
        } else {
            None
        };
        sqlx::query!(
            r#"
        INSERT INTO dynamic_commands (id, command, response, guild_id, attachment_urls)
        VALUES ($1, $2, $3, $4, $5);
            "#,
            self.id,
            self.command,
            self.response,
            self.guild_id,
            attachment_urls
        )
        .execute(conn)
        .await?;
        Ok(true)
    }

    pub async fn update(
        &mut self,
        conn: &DbConn,
        new_response: &str,
        new_attachment_urls: &Option<Vec<String>>,
    ) -> Result<bool, sqlx::Error> {
        let attachment_urls: Option<&[String]> = if let Some(urls) = new_attachment_urls {
            Some(urls.as_slice())
        } else {
            None
        };
        sqlx::query!(
            r#"
        UPDATE dynamic_commands
        SET
            response = $2,
            attachment_urls = $3
        WHERE
            id = $1;
            "#,
            self.id,
            new_response,
            attachment_urls
        )
        .execute(conn)
        .await?;

        self.response = new_response.to_string();

        Ok(true)
    }

    pub async fn rename(
        &mut self,
        conn: &DbConn,
        new_command_name: &str,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query!(
            r#"
        UPDATE dynamic_commands
        SET
            command = $2
        WHERE
            id = $1;
            "#,
            self.id,
            new_command_name,
        )
        .execute(conn)
        .await?;

        self.command = new_command_name.to_string();

        Ok(true)
    }
}

#[derive(Default, sqlx::FromRow)]
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

        Ok(true)
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
        Ok(guild_data)
    }
}
