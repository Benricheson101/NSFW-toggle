const TOGGLE_COMMAND = {
  name: 'toggle',
  description: 'Enable or disable NSFW for a channel',
  options: [
    {
      type: 7, // channel
      name: 'channel',
      description: 'The channel to modify',
      required: true,
    },
  ],
};

const PING_COMMAND = {
  name: 'ping',
  description: 'Pong!',
};

module.exports = {TOGGLE_COMMAND, PING_COMMAND};
