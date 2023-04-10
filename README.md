Experimenting with Conway's "Game of Life" in Rust.

# TODO
Provide explanation of what I've done so far, experiments tried.

# FFMPEG 

I've been using this command to translate the PNG to high quality AV1.
`ffmpeg -framerate 60 -pattern_type glob -i 'output/*.png' -c:v libsvtav1 -g 180 out.mp4`
