# Jarvis CD

![Build status](https://api.travis-ci.org/Ookami86/jarvis-cd.svg?branch=master)

Jarvis CD is a lightweight Build Server/Continuous Delivery tool for the pedantic idealist. 

## How it works

Git push something to your Repo in order to trigger a build. If the build fails, your revision will be rejected by the server by default.

:warning: This is a work in progress and not intended for production use. :warning:

![Usage example gif file](usage.gif)

## What makes it unique

Design goals for this tool are:

* Very easy setup
* Treating pipelines-as-code all the way
* Using Pre-Tested Commits by default (it's a git pre-receive hook)
* Be secure by default
* Pipelines run isolated by default. (it's based on Docker containers)

## Usage

At the moment a Jarvis build is made up of four parts:

- the [Jarvis executable](jarvis)
- a [Dockerfile](Dockerfile)
- a [Jarvisfile](Jarvisfile)
- a [pre-receive hook](pre-receive.hook) for your Git Server

#### Jarvis executable

Jarvis comes bundled with a Python runtime. All you need is a server with a halfway recent glibc (2.23 and above) and Docker. Place it in your repo alongside the other files. You can even dry-run it on your local dev machine!

#### Dockerfile

Used to build the isolated OS environment your build will execute in. You probably already have this. Put things here that don't change often, as it will only be rebuilt when the Dockerfile changes. 
The files you pushed to Git will be mounted at `/jarvis/` in the container so you will need to add a `VOLUME /jarvis/` to this Dockerfile.

#### Jarvisfile

This defines your build stages and the command to invoke at each stage. It's simple: 

    - stage: Prepare
      script: pipenv --bare install --dev
    - stage: Style check
      script: pipenv run pycodestyle src
    - stage: Test
      script: pipenv run pytest
    - stage: Package
      script: pipenv run pyinstaller jarvis.spec

#### Post-receive hook

This is where the magic happens. Put it into your Git server's hook directory (e.g. [the hooks/repo-specific directory in gitolite](https://gitolite.com/gitolite/non-core.html)) and it'll turn it into a build server. 

## Development

Install pipenv to create manage a virtualenv:

    pip install pipenv
    pipenv install --dev

Tests are run with

    pytest 

## TODO list of nice-to-add features

- [ ] secure secret storage
- [ ] deploy support
- [ ] a shim like gradlew you can commit in Git instead of the binary
- [ ] more configuration options
- [ ] limit parallel builds 
- [ ] create a decent CLI
- [ ] automate release process 
