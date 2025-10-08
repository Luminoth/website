# EnergonSoftware Website

## Global Setup

* sudo apt install docker-buildx
* Install Node.js
  * See https://angular.dev/reference/versions for compatible versions
  * Use [nvm](https://github.com/nvm-sh/nvm) to install and use the needed version
    * This doesn't seem to work with vscode, but is fine in a terminal
* npm install -g @angular/cli

## General backend updating

* cargo update

## General frontend updating

* https://angular.dev/update-guide
* cd energonsoftware
* ng update @angular/cli @angular/core @angular/material @angular/cdk
* npm update

## Yearly updating

* Update Copyright footer in `navigation.component.html`

## Notes

* Warp removed in 292496f850aebd54875d40f2a041afcb60777fac
