# Elinvar
This is a project put together by [MudCoders](http://mudcoders.com/).  The current plan is to build
a MUD using [Rust](https://www.rust-lang.org/).

### Setup Vagrant
Follow these instructions to setup [Vagrant](https://www.vagrantup.com/downloads.html) on your
operating system.

#### Linux (apt-get)
On machines with apt-get, you can use this command.

    sudo apt-get install vagrant

#### Linux (pacman)
On machines with pacman, you can use this command.

    sudo pacman -S vagrant

#### Windows
On Windows, go to the [Vagrant](https://www.vagrantup.com/downloads.html) website and download the installer.

#### Build and test the project
    git clone the project
    cd /dir/with/vagrantfile
    vagrant up
    vagrant ssh
    cd /vagrant
    make
    make test

`vagrant up`: This command looks at the vagrantfile and downloads/builds a VM based on the specifications
of the file.  It also provisions and installs a set of tools used by the project.  Finally, it mounts
the local folder to the `/vagrant` folder on the VM.

`vagrant ssh`: This command opens an SSH shell on the VM and allows you to run commands on the VM.
