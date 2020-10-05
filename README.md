# compose-updater

![CI](https://github.com/RealOrangeOne/compose-updater/workflows/CI/badge.svg)

Simple command to enable updating docker-compose based applications.

If newer containers are available than those running, pull the new containers, and cycle the containers. Supports specifying multiple containers using globs, and multiple globs if necessary.

## Features

- Only cycle application if containers were updated (unless `--force-cycle` specified)
- Don't try and update stopped containers (unless `--force-pull` specified)

## _"How does it work?"_

- List images
- If there are images, pull (`docker-compose pull`)
- List images again
- If images list is different, cycle container (`docker-compose down && docker-compose up -d`)
