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

from nextcord.ext import commands  # type: ignore[attr-defined]


class Bot(commands.Cog):
    """Various commands related to the bot"""

    def __init__(self, bot: commands.Bot) -> None:
        self.bot = bot

    @commands.command()
    async def ping(self, ctx: commands.Context) -> None:
        """Prints bot's latency"""
        await ctx.send(f"My ping is: `{int(self.bot.latency * 1000)}`ms")


def setup(bot: commands.Bot) -> None:
    bot.add_cog(Bot(bot))
