const Discord = require("discord.js");
const client = new Discord.Client({
	disableEveryone: true
});
const fetch = require("node-fetch");
require("dotenv").config();


client.on("ready", async () => {
	console.info(`${client.user.username} is now online!
    Guilds: ${client.guilds.size}
    Channels: ${client.channels.size}
    Users: ${client.users.size}`);
	await client.user.setPresence({
		game: {
			name: `for @${client.user.username} toggle`,
			type: "WATCHING"
		},
		status: "dnd"
	});

// bot list stuff
if (process.env.NODE_ENV === "production" && process.env.BOD_KEY) {
  setInterval(() => {
    fetch(`https://bots.ondiscord.xyz/bot-api/bots/${client.user.id}/guilds`, {
      method: "post",
      headers: {
        Authorization: process.env.BOD_KEY,
        "Content-Type": "application/json"
      },
      body: JSON.stringify({ guildCount: client.guilds.size })
    })
      .then(console.log)
      .catch(client.error);
  }, 120000);
}

});

client.on("message", async (message) => {
	if (message.author.bot || message.channel.type !== "text") return;
	if (!message.member.hasPermission(["MANAGE_CHANNELS"], false, true, true)) return;
	let command = message.content.split(" ");
  const prefixRegex = new RegExp(`^<@!?${client.user.id}>`);
  if (!prefixRegex.test(command[0])) return;

  const subcmd = command.join(" ").replace(prefixRegex, "".split(" ")).trim();
	switch (subcmd) {
	case ("toggle-nsfw"):
	case ("toggle"):
	case ("nsfw"): {
		if (!message.guild.me.hasPermission(["MANAGE_CHANNELS"], false, true, true)) {
			message.channel.send(":x: I do not have permission to toggle NSFW for this channel. Please make sure that I have `MANAGE_CHANNELS`")
				.then((m) => m.delete(5000));
			break;
		}
		if (message.guild.me.hasPermission(["MANAGE_MESSAGES"])) message.delete();
		try {
			await message.channel.setNSFW(!(message.channel.nsfw), `Requested by: ${message.author.username}#${message.author.discriminator} (${message.author.id})`);
		} catch (e) {
			if (e.message === "Missing Permissions") {
				message.channel.send(":x: I do not have permission to toggle NSFW for this channel. Please make sure that I have `MANAGE_CHANNELS`");
				break;
			}
			console.error(e);
			message.channel.send(":x: An unhandled error occurred. Please try again.");
			break;
		}
		await message.channel.send(`Successfully ${message.channel.nsfw ? "" : "un"}marked <#${message.channel.id}> as NSFW.`)
			.then((m) => m.delete(5000))
			.catch((e) => {
				if (e.message !== "Unknown Message") return console.error(e);
			});
		break;
	}
	default: {
		message.channel.send(":x: Unknown command.")
			.then((m) => m.delete(5000));
		break;
	}
	}
});

client.login(process.env.TOKEN)
	.catch(console.error);
