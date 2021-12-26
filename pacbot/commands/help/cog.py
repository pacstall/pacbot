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

from typing import Dict, Iterable, Optional, Set

import nextcord.ext.commands as commands
from nextcord import Embed


class MyHelpCommand(commands.MinimalHelpCommand):
    def get_command_signature(self, command: commands.Command) -> str:
        return (
            f"{self.context.clean_prefix}{command.qualified_name} {command.signature}"
        )

    async def _help_embed(
        self,
        title: str,
        description: Optional[str] = "",
        mapping: Optional[Dict[commands.Cog, Set[commands.Command]]] = {},
        command_set: Optional[Iterable[commands.Command]] = None,
    ) -> Embed:
        embed = Embed(title=title, description=description)

        embed.set_author(
            name=self.context.bot.user.name,
            icon_url=(
                self.context.bot.user.avatar or self.context.bot.user.default_avatar
            ),
        )

        if command_set:
            # show help about all commands in the set
            for command in await self.filter_commands(command_set, sort=True):
                embed.add_field(
                    name=self.get_command_signature(command),
                    value=command.short_doc or "...",
                    inline=False,
                )

        if mapping:
            for cog, command_set in mapping.items():
                if not (filtered := await self.filter_commands(command_set, sort=True)):
                    continue

                name = cog.qualified_name if cog else "No category"

                # \u2002 is an en-space
                cmd_list = "\u2002".join(
                    f"`{self.context.clean_prefix}{cmd.name}`" for cmd in filtered
                )

                value = (
                    f"{cog.description}\n{cmd_list}"
                    if cog and cog.description
                    else cmd_list
                )

                embed.add_field(name=name, value=value)
        return embed

    async def send_bot_help(
        self, mapping: Dict[commands.Cog, Set[commands.Command]]
    ) -> None:
        await self.get_destination().send(
            embed=await self._help_embed(
                title="Bot Commands",
                description=self.context.bot.description,
                mapping=mapping,
            )
        )

    async def send_command_help(self, command: commands.Command) -> None:
        await self.get_destination().send(
            embed=await self._help_embed(
                title=self.get_command_signature(command),
                description=command.help,
                command_set=command.commands
                if isinstance(command, commands.Group)
                else None,
            )
        )

    async def send_cog_help(self, cog: commands.Cog) -> None:
        await self.get_destination().send(
            embed=await self._help_embed(
                title=cog.qualified_name,
                description=cog.description,
                command_set=cog.get_commands(),
            )
        )


class Help(commands.Cog):
    """Shows help info about commands"""

    def __init__(self, bot: commands.Bot) -> None:
        self._original_help_command = bot.help_command
        bot.help_command = MyHelpCommand()
        bot.help_command.cog = self
        self.bot = bot

    def cog_unload(self) -> None:
        self.bot.help_command = self._original_help_command


def setup(bot: commands.Bot) -> None:
    bot.add_cog(Help(bot))
