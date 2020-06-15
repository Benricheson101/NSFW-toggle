import 'dotenv/config.js'
import {
  Client,
  MessageEmbed,
  Intents
} from 'discord.js'
import fetch from 'node-fetch'

const client = new Client({
  disableMentions: 'all',
  ws: {
    intents: new Intents(['GUILD_MESSAGES', 'GUILDS'])
  }
})

client.commandsUsed = { help: 0, toggle: 0 }

client.on('ready', async () => {
  client.user.setPresence({
    activity: {
      type: 'PLAYING',
      name: `@${client.user.username} help | ${client.guilds.cache.size} servers`
    },
    status: 'dnd'
  })

  console.log(`Ready! Logged in as ${client.user.tag}\nServers: ${client.guilds.cache.size}`)

  if (process.env.NODE_ENV === 'production') {
    postToList(client)
    setInterval(() => {
      postToList(client)
    }, 120000)
  }
})

client.on('message', async (msg) => {
  const prefix = new RegExp(`^<@!?${client.user.id}>`)
  if (
    msg.author.bot ||
    msg.channel.type !== 'text' ||
    !prefix.test(msg.content)
  ) return

  const command = msg.content.replace(prefix, '').trim().toLowerCase()

  const helpCommands = [
    'help',
    'howto',
    'how-to',
    'how_to',
    'commands'
  ]

  const toggleCommands = [
    'toggle',
    'switch',
    'enable',
    'disable',
    'nsfw',
    'nsfwtoggle',
    'nsfw-toggle',
    'nsfw_toggle',
    'togglensfw',
    'toggle-nsfw',
    'toggle_nsfw',
    'nsfwon',
    'nsfw-on',
    'nsfw_on',
    'nsfwoff',
    'nsfw-off',
    'nsfw_off'
  ]

  if (helpCommands.some((c) => command.startsWith(c))) {
    client.commandsUsed.help++
    return msg.channel.send(
      new MessageEmbed()
        .setColor('#0f4275')
        .addField('Toggle', `
**Description**: Toggle NSFW
**Commands**:
${toggleCommands.map((c) => '`' + c + '`').join('\n')}
**Used**: ${client.commandsUsed.toggle} times since the last restart
        `)
        .addField('Help', `
**Description**: Get a list of commands
**Commands**:
${helpCommands.map((c) => '`' + c + '`').join('\n')}
**Used**: ${client.commandsUsed.help} times since the last restart
        `)
        .setFooter('Note: users must have `MANAGE_CHANNELS` or higher to use any of the bot\'s commands')
    )
  }

  if (toggleCommands.some((c) => command.startsWith(c))) {
    client.commandsUsed.toggle++

    const args = msg.content.split(' ').slice(2)

    let channel = msg.channel

    if (args[0]) {
      channel = msg.mentions.channels.first() ||
        msg.guild.channels.cache.find((c) => c.id === args[0] || c.name.toLowerCase() === args.join('-').toLowerCase())
    }

    if (!channel) {
      return await msg.channel.send(`:x: I was not able to find a channel with the name or ID: \`${args.join(' ')}\``)
        .then((m) => m.delete({ timeout: 5000 }))
    }

    if (
      !msg.guild.me.permissions.has('MANAGE_CHANNELS') ||
      !msg.channel.permissionsFor(msg.guild.me).has('MANAGE_CHANNELS')
    ) {
      return await msg.channel.send(':x: I do not have permission to toggle NSFW. Please make sure I have `MANAGE_CHANNELS`')
        .then((m) => m.delete({ timeout: 5000 }))
    }

    if (!msg.member.permissionsIn(channel).has('MANAGE_CHANNELS')) {
      return await msg.channel.send(':x: You do not have permission to use this command. You must have `MANAGE_CHANNELS`')
        .then((m) => m.delete({ timeout: 5000 }))
    }

    channel.edit({ nsfw: !channel.nsfw })
      .then((c) => {
        msg.channel.send(`:white_check_mark: Channel \`${channel.name}\` ${c.nsfw ? 'marked' : 'unmarked'} as NSFW.`)
          .then((m) => m.delete({ timeout: 5000 }))
        if (msg.guild.me.permissions.has('MANAGE_MESSAGES')) msg.delete()
      })
      .catch(async (err) => {
        if (err.message === 'Missing Permissions') {
          return await msg.channel.send(':x: I do not have permission to toggle NSFW. Please make sure I have `MANAGE_CHANNELS`')
            .then((m) => m.delete({ timeout: 5000 }))
        }

        return await msg.channel.send(`:x: An error occurred. \n \`\`\`\n${err}\`\`\``)
      })
  }
})

client.login(process.env.TOKEN)

function postToList (client) {
  return fetch(`https://bots.ondiscord.xyz/bot-api/bots/${client.user.id}/guild`, {
    method: 'post',
    headers: { authorization: process.env.BOD_KEY },
    body: JSON.stringify({ guildCount: client.guilds.cache.size })
  })
}
