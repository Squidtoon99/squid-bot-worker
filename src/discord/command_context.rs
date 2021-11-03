use crate::context::Context;
use crate::discord::prelude::*;
use twilight_model::application::interaction::InteractionType;
use twilight_model::guild::PartialMember;
use twilight_model::user::User;

pub(crate) struct CommandContext {
    context: Context,
    pub application_id: ApplicationId,
    pub channel_id: ChannelId,
    pub data: CommandData,
    pub guild_id: Option<GuildId>,
    pub id: InteractionId,
    pub kind: InteractionType,
    pub member: Option<PartialMember>,
    pub token: String,
    pub user: Option<User>,
}

impl CommandContext {
    pub fn new(ctx: &Context, interaction: &ApplicationCommand) -> Self {
        let i = interaction.clone();
        Self {
            context: ctx.clone(),
            application_id: i.application_id,
            channel_id: i.channel_id,
            data: i.data,
            guild_id: i.guild_id,
            id: i.id,
            kind: i.kind,
            member: i.member,
            token: i.token,
            user: i.user,
        }
    }
    pub fn command_name(&self) -> String {
        // There might be subcommands in the options so we deconstruct them
        // Until I feel stupid enough to nest more than 1 subcommand I'm making this simple on myself
        let mut names = Vec::<String>::new();
        
        names.push(self.data.name.clone());
        if self.is_subcommand() {
            let mut opts = &self.data.options;
            while let Some(CommandOptionValue::SubCommandGroup(s_opts)) =
                opts.first().map(|o| &o.value)
            {
                if let Some(name) = opts.first().map(|o| o.name.clone()) {
                    names.push(name);
                };
                opts = &s_opts;
            }
        };
        names.join(" ")
    }

    pub fn data(&self) -> &CommandData {
        &self.data
    }
    
    pub fn is_subcommand(&self) -> bool {
        if let Some(val) = &self.data.options.first().map(|o| &o.value) {
            if let CommandOptionValue::SubCommand(_) = val {
                true
            } else if let CommandOptionValue::SubCommandGroup(_) = val {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn arguments(&self) -> &Vec<CommandDataOption> {
        let mut opts = &self.data.options;

        while let Some(CommandOptionValue::SubCommandGroup(s_opts)) =
            &opts.first().map(|o| &o.value)
        {
            opts = &s_opts;
        }

        if let Some(CommandOptionValue::SubCommand(f_opts)) = &opts.first().map(|o| &o.value) {
            &f_opts
        } else {
            opts
        }
    }

    pub fn env(&self, name: &str) -> Option<&String> {
        match self.context.env(name) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn redis(&self) -> RedisClient {
        self.context.new_redis()
    }
}
