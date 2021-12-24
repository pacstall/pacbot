#!/usr/bin/env python3

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

import logging
from os import getenv

from dotenv import load_dotenv
from nextcord.ext import commands  # type: ignore[attr-defined]

load_dotenv()

bot = commands.Bot(command_prefix="$")

# Setup logging
logger = logging.getLogger("nextcord")
logger.setLevel(logging.DEBUG)
handler = logging.FileHandler(filename="nextcord.log", encoding="utf-8", mode="w")
handler.setFormatter(
    logging.Formatter("%(asctime)s:%(levelname)s:%(name)s: %(message)s")
)
logger.addHandler(handler)


@bot.event
async def on_ready() -> None:
    print(f"Successfully logged in as {bot.user}")


@bot.command()
async def hello(ctx: commands.Context) -> None:
    await ctx.send(f"Hello {ctx.author.mention}")


bot.run(getenv("BOT_TOKEN"))
