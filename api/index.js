require('dotenv/config');

const {PING_COMMAND, TOGGLE_COMMAND} = require('../src/cmds');
const {createCommands} = require('../src/createCmds');

const CLIENT_SECRET = process.env.CLIENT_SECRET;
const CLIENT_ID = process.env.CLIENT_ID;

createCommands([PING_COMMAND, TOGGLE_COMMAND], {
  appId: CLIENT_ID,
  clientSecret: CLIENT_SECRET,
  guild: process.env.GUILD,
});

module.exports = require('../src/handleCmds');
