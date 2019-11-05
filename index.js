const Discord = require("discord.js");
const client = new Discord.Client({
	disableEveryone: true
});
require("dotenv").config();

client.on("ready", async () => {
	console.info(`${client.user.username} is now online!`);
	await client.user.setPresence({
		game: {
			name: `for @${client.user.username} nsfw`,
			type: "WATCHING"
		},
		status: "dnd"
	});
});

client.on("message", async (message) => {
	if (message.author.bot) return;
	let command = message.content.split(" ");
	if (command[0] !== `<@${client.user.id}>`) return;

	switch (command.slice(1).join()) {
	case ("toggle-nsfw"):
	case ("toggle"):
	case ("nsfw"): {
		if (!message.member.hasPermission(["MANAGE_CHANNELS"], false, true, true)) break;
		if (message.guild.me.hasPermission(["MANAGE_MESSAGES"])) message.delete();

		await message.channel.setNSFW(!(message.channel.nsfw), `Requested by: ${message.author.username}#${message.author.discriminator} (${message.author.id})`)
			.then(
				await message.channel.send(`Successfully ${message.channel.nsfw ? "unmarked" : "marked"} <#${message.channel.id}> as NSFW.`)
					.then((m) => m.delete(5000))
					.catch((e) => {
						if (e.message !== "Unknown Message") return console.error(e);
					})
			);
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
