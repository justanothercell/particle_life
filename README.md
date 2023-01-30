## Windows specific SDL2 info
Download each the `*-2.6.2-VC.zip` from
[SDL](https://github.com/libsdl-org/SDL/releases/tag/release-2.26.2) and
[SDL_ttf](https://github.com/libsdl-org/SDL_ttf)
respectively and add each `lib/x64` to the `LIB` environment variable.

Copy `SDL2.dll` and `SDL2_ttf.dll` into the root of your project (already done).
If necessary replace with a version compatible to your download.

`SDL2_gfx.dll` got acquired from this [Google Drive](https://drive.google.com/drive/folders/14RPWmR-xOE30aUnZnOy0hT9AzZejNkqs)<br>
`SDL2_gfx.lib` was built manually for windows x64 and is in local files instead of on `PATH` so you don't have to.