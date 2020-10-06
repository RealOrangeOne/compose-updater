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

## _"How's this different from `pull` and `up -d`?"_

After writing this, turns out that `docker-compose` support doing most of this for you.

> If there are existing containers for a service, and the service’s configuration or image was changed after the container’s creation, `docker-compose up` picks up the changes by stopping and recreating the containers (preserving mounted volumes). 

Therefore, you can easily achieve this by simply pulling each compose file, and running `up -d`. Which changes all this Rust into [a handful of lines of Bash]( https://github.com/RealOrangeOne/infrastructure/blob/master/ansible/roles/docker_cleanup/files/docker-utils/update-all).
