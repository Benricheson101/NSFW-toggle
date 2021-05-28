const axios = require('axios');
const {stringify} = require('querystring');

/**
 * @typedef CreateCommadsOptions
 * @property {string} clientSecret The client secret used to create commands
 * @property {string} appId THe client ID
 * @property {string | null} guild Where to create the commands
 */

/**
 * Create slash commands
 * @param {object[]} commands An array of commands to create
 * @param {CreateCommadsOptions} options Command creation options
 */
async function createCommands(commands, {appId, clientSecret, guild = null}) {
  const body = {
    grant_type: 'client_credentials',
    scope: 'applications.commands.update',
  };

  const headers = {
    'Content-Type': 'application/x-www-form-urlencoded',
  };

  const auth = {
    username: appId,
    password: clientSecret,
  };

  const res = await axios.post(
    'https://discord.com/api/v8/oauth2/token',
    stringify(body),
    {headers, auth}
  );

  if (!res.data?.access_token) {
    console.error('Unable to get bearer token from request');
    process.exit(1);
  }

  try {
    if (guild) {
      await axios.put(
        `https://discord.com/api/v9/applications/${appId}/guilds/${guild}/commands`,
        JSON.stringify(commands),
        {
          headers: {
            authorization: `Bearer ${res.data.access_token}`,
            'Content-Type': 'application/json',
          },
        }
      );
    }
  } catch (err) {
    console.dir(err.response, {depth: null});
  }
}

module.exports = {createCommands};
