# The Wind Waker Beta Quest

Rom Hack for The Legend of Zelda: The Wind Waker that randomizes the warps.

## How To Compile

To compile the source code you need to get the Rust Nightly compiler toolchain.
You can acquire it either through the [official website](https://www.rust-lang.org/downloads.html) or through [multirust](https://github.com/brson/multirust).

If you use multirust you might need to override the toolchain in the project by using ```multirust override nightly```.

Since we'll compile PowerPC code, you'll need to get a compiled ```libcore``` for PowerPC.
Luckily, there's a prebuilt version available [here](http://static.rust-lang.org/dist/rust-std-nightly-powerpc-unknown-linux-gnu.tar.gz).

We'll need to target the GameCube specifically, so we'll need a special linker which you can get by installing [DevkitPPC](http://devkitpro.org/wiki/Getting_Started/devkitPPC).

Now that we have the whole toolchain, you will need to unpack your version of Wind Waker (GZLJ01) into a new folder called ```game```.
You can use the [GameCube ISO Tool](http://www.wiibackupmanager.co.uk/gcit.html) for that.
The ```game``` folder should contain the following folders if done correctly: ```root``` and ```sys```.
In the ```sys``` folder you can find a ```main.dol```.
This is the main executable of the game and will be the one we compile into.
We'll need to create a backup of the file called ```original.dol``` that you put directly into the game folder.

The folder structure should look like this now:

 - game
   - sys
   - root
   - original.dol
 - libtww
 - patcher
 - src
 - ...

At this point you can execute the ```make``` command.
It will fail to fully execute for now.
Go into the ```src/target/powerpc-unknown-linux-gnu/release``` folder and create a link to ```librust.a``` located in a new folder called ```lib``` in the root folder.
On Windows the library might have the extension ```.lib``` instead.

The folder structure should look like this now:

 - build
 - game
   - sys
   - root
   - original.dol
 - lib
   - librust.a
 - libtww
 - patcher
 - src
 - ...

Execute ```make``` again and it should compile now.
The compiled executable will be located in the ```game/sys/``` folder.
You can use the GameCube ISO Tool to convert this into an ISO again or directly boot up the folder with Dolphin.
