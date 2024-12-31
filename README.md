# kamarust
A basic example of a Rust web app deployment with the excellent [Kamal](https://kamal-deploy.org/)

# Instructions

1. Copy the example config:
   ```sh
   cp config/deploy.example.yml config/deploy.yml
   ```

2. Edit `config/deploy.yml` to suit your requirements.

3. Install Kamal if you don't have it already or use docker if the limitations of using it that way are acceptable to you - [more info here](https://kamal-deploy.org/docs/installation/dockerized/).

4. Run `kamal init`

5. Set your `KAMAL_REGISTRY_PASSWORD` in your environment and edit your .kamal/secrets file to read it. There are a variety of ways to do this - the 1Password integration is particularly neat.

   If using docker you can hard-code your secrets in that file but be sure to add it to `.gitignore` and NEVER commit it to git!

6. Now you can deploy with:
   `kamal setup`
