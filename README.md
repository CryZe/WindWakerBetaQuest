# The Wind Waker Beta Quest

Rom Hack for The Legend of Zelda: The Wind Waker that randomizes the warps.

## How To Compile

To compile the source code you need to get the Rust Nightly compiler toolchain.
You can acquire it either through the [rustup](https://rustup.rs/).

If you use rustup you might need to override the toolchain in the project by using `rustup override nightly`.

Since we'll compile PowerPC code, you'll need to get a compiled `libcore` for PowerPC.
You can install it with `rustup target install powerpc-unknown-linux-gnu`.

We'll need to target the GameCube specifically, so we'll need a special linker which you can get by installing [DevkitPPC](http://devkitpro.org/wiki/Getting_Started/devkitPPC).

Now that we have the whole toolchain, you will need to unpack your version of Wind Waker (GZLJ01) into the folder called `game`.
You can use the [GameCube ISO Tool](http://www.wiibackupmanager.co.uk/gcit.html) that comes with this repository for that.
The `game` folder should contain the following folders if done correctly: `root` and `sys`.
In the `sys` folder you can find a `main.dol`.
This is the main executable of the game and will be the one we compile into.
We'll need to create a backup of the file called `original.dol` that you put directly into the game folder.

The folder structure should look like this now:

 - game
   - sys
   - root
   - original.dol
 - libtww
 - src
 - ...

At this point you can execute the `make` command.
Execute `make` again and it should compile now.
The compiled executable will be located in the `game/sys/` folder.

If you want, you can use `make iso` to compile it into an ISO or `make cheat` to compile it into a cheat.
