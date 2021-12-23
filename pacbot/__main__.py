from os import getenv

import nextcord
from dotenv import load_dotenv

load_dotenv()

client = nextcord.Client()


@client.event
async def on_ready():
    print(f"Successfully logged in as {client.user}")


@client.event
async def on_message(message):
    if message.author == client.user:
        return

    if message.content.startswith("hello"):
        await message.channel.send("Hello to you!")


client.run(getenv("BOT_TOKEN"))
