import 'dotenv/config.js'
import { ShardingManager } from 'discord.js'

const manager = new ShardingManager('./src/index.js', { token: process.env.TOKEN })
manager.spawn()
  .catch(console.error)
