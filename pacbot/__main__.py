#     ____             ____        __
#    / __ \____ ______/ __ )____  / /_
#   / /_/ / __ `/ ___/ __  / __ \/ __/
#  / ____/ /_/ / /__/ /_/ / /_/ / /_
# /_/    \__,_/\___/_____/\____/\__/
#
# Copyright (C) 2021-Present
#
# This file is part of PacBot.
#
# PacBot is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# PacBot is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with PacBot.  If not, see <https://www.gnu.org/licenses/>.

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
