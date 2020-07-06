# discord dev cli (ddc)
a cli application to interact with the discord developers portal

## is this self-botting?
technially, yes. although you *should* be fine.
note: the license waives all liability

## does this work?
not really, no, at least not yet.

## will there be some sort of GUI?
I have hopes of using [fdehau/tui-rs](https://github.com/fdehau/tui-rs)

## how will this handle authentication?
the first time running you'll need to provide a token  
every time you run the program it'll fetch `/users/@me`, more or less to ensure the token is still valid
