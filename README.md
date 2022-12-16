</p>
<p align="center">
<a href="https://github.com/pacstall/pacbot"><img align="center" src="https://raw.githubusercontent.com/pacstall/website/master/client/public/pacstall.svg" alt="Pacstall Logo" width="200" height="200" loading="lazy"></a>
</p>
<p align="center"><b>PacBot</b></p>
</p>

---

#### How to deploy
Create a file called `.env`, and fill in the following information:
```bash
DISCORD_TOKEN=$my_discord_token
PACSTALL_GUILDID=$my_guild_id
```

Then run `docker-compose up -d` to build the container.
