if (process.env.NODE_ENV === 'dev') {
  require('dotenv/config');
}

const {
  verifyKey,
  InteractionType,
  InteractionResponseType,
} = require('discord-interactions');

const axios = require('axios');

const {PING_COMMAND, TOGGLE_COMMAND} = require('./cmds');
const {createCommands} = require('./createCmds');

const PUBLIC_KEY = process.env.PUBLIC_KEY;
const DISCORD_TOKEN = process.env.DISCORD_TOKEN;
const CLIENT_SECRET = process.env.CLIENT_SECRET;
const CLIENT_ID = process.env.CLIENT_ID;

createCommands([PING_COMMAND, TOGGLE_COMMAND], {
  appId: CLIENT_ID,
  clientSecret: CLIENT_SECRET,
  guild: process.env.GUILD,
});

module.exports = async (req, res) => {
  switch (req.method) {
    case 'GET': {
      return res.redirect('<invite here>');
    }

    case 'POST': {
      const sig = req.headers['x-signature-ed25519'];
      const time = req.headers['x-signature-timestamp'];
      const body = JSON.stringify(req.body);

      const isValid = verifyKey(body, sig, time, PUBLIC_KEY);

      if (!isValid) {
        return res.status(401);
      }

      const msg = req.body;

      if (msg.type === InteractionType.PING) {
        return res.send({
          type: InteractionResponseType.PONG,
        });
      } else if (msg.type === InteractionType.APPLICATION_COMMAND) {
        switch (msg.data.name.toLowerCase()) {
          // /toggle channel: 123
          case TOGGLE_COMMAND.name: {
            // eslint-disable-next-line new-cap
            const perms = BigInt(msg.member.permissions);

            if ((perms & (1n << 4n)) !== 1n << 4n) {
              return res.status(200).send({
                type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
                data: {
                  content:
                    ':x: You do not have permission to use this command. You must have at least `MANAGE_CHANNELS`',
                  flags: 64,
                },
              });
            }

            const channel = msg.data.options.find(o => o.name === 'channel');

            if (!channel) {
              return;
            }

            const headers = {
              Authorization: `Bot ${DISCORD_TOKEN}`,
            };

            try {
              const toUpdate = await axios.get(
                `https://discord.com/api/v9/channels/${channel.value}`,
                {headers}
              );

              if (toUpdate?.data?.type !== 0) {
                return res.status(200).send({
                  type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
                  data: {
                    content: ':x: NSFW can only be toggled for text channels',
                    flags: 64,
                  },
                });
              }

              const result = await axios.patch(
                `https://discord.com/api/v9/channels/${channel.value}`,
                {nsfw: !toUpdate.data.nsfw},
                {headers}
              );

              let m =
                ':x: For some reason I was unable to verify whether or not NSFW was toggled.';
              if (result?.data?.nsfw === !toUpdate.data.nsfw) {
                m = `:white_check_mark: Successfully **${
                  result.data.nsfw ? 'enabled' : 'disabled'
                }** NSFW for <#${result.data.id}>`;
              }

              return res.status(200).send({
                type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
                data: {
                  content: m,
                  flags: 64,
                },
              });
            } catch (e) {
              let errorMessage =
                ':x: An error occurred: `' + e.response.data.message + '`';

              if ([50_001, 50_013].includes(e?.response?.data?.code)) {
                errorMessage =
                  ':x: I am missing permissions in that channel. Please make sure I have both `Read Messages` and `Manage Channel`';
              } else if (e?.response?.data) {
                console.error(e.response.data);
              }

              return res.status(200).send({
                type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
                data: {
                  content: errorMessage,
                  flags: 64,
                },
              });
            }

            break;
          }

          // /ping
          case PING_COMMAND.name.toLowerCase(): {
            return res.status(200).send({
              type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
              data: {
                content: 'Pong!',
                flags: 64,
              },
            });
          }
        }
      }

      break;
    }

    default: {
      return res.status(405).send('Method Not Allowed');
    }
  }
};
