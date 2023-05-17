# Vue.js Docker Template

## Description
This project is a template to setup vue.js projects via __docker__ and __docker-compose__.
## Steps
- clone this project or [download the project-files](https://git.ufz.de/my-projects/vuejs-docker-template/-/archive/main/vuejs-docker-template-main.zip)
- open terminal cd `cd` into the project
- run `chmod +x ./bin/go-to-node-container.sh`
- run `chmod +x ./bin/create-vue-project.sh`
- run `chmod +x ./bin/docker-run.sh`
- run `./bin/create-vue-project.sh`
- answer the questions if you know what you're doing or use the following answers:
  - Your connection to the default yarn registry seems to be slow.
    Use https://registry.npm.taobao.org for faster installation? (Y/n): __`n`__
  - Generate project in current directory? (Y/n): __`Y`__
  - Please pick a preset: __`Default`__
  - Pick the package manager to use when installing dependencies: __`NPM`__
- `docker-compose up` or `docker-compose up -d`
- app runs on: `http://localhost:3000`
- _(optional)_ add files in `./app` to git if needed

## Helper scripts
- __`create-vue-project.sh`__
  - creates a new vue.js project in `./app` directory 
- __`go-to-node-container.sh`__
  - if you need to go to the command line of the container you can use this script
- __`docker-run.sh`__
  - use this script to run any command inside the container, e.g.:
    - `./bin/docker-run.sh npm install that-fancy-package`
    - `./bin/docker-run.sh vue add vuetify`

## Additional note
I think you must always create an `app` directory first, before creating a new vue.js project. I ran into some permission problems, when I forgot that and docker/container created the directory by itself, e.g., you can only delete a file with `sudo`.  