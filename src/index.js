import {
  Client,
  MessageEmbed,
  Intents
} from 'discord.js'
import { promisify } from 'util'
import fetch from 'node-fetch'

const wait = promisify(setTimeout)

const client = new Client({
  disableMentions: 'all',
  ws: {
    intents: new Intents(['GUILD_MESSAGES', 'GUILDS'])
  }
})

client.commandsUsed = 0

client.on('ready', async () => {
  const setPresence = async () => client.user.setPresence({
    activity: {
      type: 'PLAYING',
      name: `@${client.user.username} help | ${await totalGuilds()} servers`
    },
    status: 'dnd'
  })

  console.log(`[${client.shard.ids[0]}] Connected.`)

  runAtInterval(setPresence, 60000)

  if (process.env.NODE_ENV === 'production' && client.shard.ids[0] === 0) {
    if (client.uptime >= 30000) await wait(30000) // wait for shards to spawn
    runAtInterval(() => postToList(client), 120000)
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
    'commands'
  ]

  const toggleCommands = [
    'toggle',
    'nsfw'
  ]

  if (toggleCommands.some((c) => c === command)) {
    client.commandsUsed++

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
  } else if (helpCommands.some((c) => command === c)) {
    client.commandsUsed++
    const embed = new MessageEmbed()
      .addFields([{
        name: 'toggle [channel]',
        value: '> **Description**: Toggle NSFW\n> **User Permission**: `MANAGE_CHANNELS`\n> **Alias**: `nsfw`'
      }, {
        name: 'help',
        value: '> **Description**: See a list of commands and what they do\n> **Alias**: `commands`'
      }, {
        name: 'invite',
        value: '> **Description**: Invite the bot'
      }, {
        name: 'support',
        value: '> **Description**: Join the support server'
      }])
      .setColor('#0f4275')
      .setFooter(`Shard: ${client.shard.ids[0]}/${client.shard.count - 1} | Servers: ${await totalGuilds()} | Commands Used: ${await totalCmdsUsed()}`)

    return msg.channel.send(embed)
  } else if (command === 'invite') {
    client.commandsUsed++
    return msg.channel.send(
      `https://discord.com/oauth2/authorize?client_id=${client.user.id}&permissions=2064&scope=bot`
    )
  } else if (command === 'support') {
    client.commandsUsed++
    if (!process.env.SUPPORT_INVITE_CODE) return
    return msg.channel.send(`https://discord.gg/${process.env.SUPPORT_INVITE_CODE}`)
  }
})

client.login(process.env.TOKEN)

async function postToList (client) {
  return fetch(`https://bots.ondiscord.xyz/bot-api/bots/${client.user.id}/guilds`, {
    method: 'post',
    headers: { authorization: process.env.BOD_KEY },
    body: JSON.stringify({ guildCount: await totalGuilds() })
  })
}

function totalGuilds () {
  return client.shard.broadcastEval('this.guilds.cache.size')
    .then((cu) => cu.reduce((prev, curr) => prev + curr), 0)
}

function totalCmdsUsed () {
  return client.shard.broadcastEval('this.commandsUsed')
    .then((cu) => cu.reduce((prev, curr) => prev + curr), 0)
}

function runAtInterval (fn, interval) {
  fn()
  setTimeout(fn, interval)
}
